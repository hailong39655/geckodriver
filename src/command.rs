use std::collections::TreeMap;
use serialize::json;
use serialize::{Decodable, Encodable};
use serialize::json::{ToJson};
use regex::Captures;

use hyper::method;
use hyper::method::Method;

use common::{WebDriverResult, WebDriverError, Status, UnknownError};
use messagebuilder::{MessageBuilder, MatchType, MatchNewSession, MatchGet, MatchGetCurrentUrl, MatchGoBack, MatchGoForward, MatchRefresh, MatchGetTitle};


#[deriving(PartialEq)]
pub enum WebDriverCommand {
    GetMarionetteId,
    NewSession,
    Get(GetParameters),
    GetCurrentUrl,
    GoBack,
    GoForward,
    Refresh,
    GetTitle
}

pub struct WebDriverMessage {
    pub command: WebDriverCommand,
    pub session_id: Option<String>
}

impl WebDriverMessage {
    pub fn new(command: WebDriverCommand, session_id: Option<String>) -> WebDriverMessage {
        WebDriverMessage {
            command: command,
            session_id: session_id
        }
    }

    pub fn name(&self) -> String {
        match self.command {
            GetMarionetteId => "getMarionetteID",
            NewSession => "newSession",
            Get(_) => "get",
            GetCurrentUrl => "getCurrentUrl",
            GoBack => "goBack",
            GoForward => "goForward",
            Refresh => "refresh",
            GetTitle => "getTitle"
        }.to_string()
    }

    fn parameters_json(&self) -> json::Json {
        match self.command {
            Get(ref x) => {
                x.to_json()
            },
            _ => {
                json::Object(TreeMap::new())
            }
        }
    }

    pub fn from_http(match_type: MatchType, params: &Captures, body: &str) -> WebDriverResult<WebDriverMessage> {
        let session_id = WebDriverMessage::get_session_id(params);
        let command = match match_type {
            MatchNewSession => NewSession,
            MatchGet => {
                let parameters_result: Result<GetParameters, json::DecoderError> = json::decode(body);
                match parameters_result {
                    Ok(parameters) => Get(parameters),
                    Err(_) => {
                        return Err(WebDriverError::new(None,
                                                       UnknownError,
                                                       "Failed to decode request body"));
                    }
                }
            },
            MatchGetCurrentUrl => GetCurrentUrl,
            MatchGoBack => GoBack,
            MatchGoForward => GoForward,
            MatchRefresh => Refresh,
            MatchGetTitle => GetTitle
        };
        Ok(WebDriverMessage {
            session_id: session_id,
            command: command
        })
    }

    fn get_session_id(params: &Captures) -> Option<String> {
        let session_id_str = params.name("sessionId");
        if session_id_str == "" {
            None
        } else {
            Some(session_id_str.to_string())
        }
    }
}

impl ToJson for WebDriverMessage {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("name".to_string(), self.name().to_json());
        data.insert("parameters".to_string(), self.parameters_json());
        data.insert("sessionId".to_string(), self.session_id.to_json());
        json::Object(data)
    }
}

#[deriving(Decodable, Encodable, PartialEq)]
struct GetParameters {
    url: String
}

impl ToJson for GetParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("url".to_string(), self.url.to_json());
        json::Object(data)
    }
}
