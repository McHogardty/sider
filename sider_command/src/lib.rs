
// use phf::Map;
// use phf_macros::phf_map;

// use crate::parser::RESPType;

mod resp;
 
pub use self::resp::RESPType;


#[derive(Debug)]
pub enum RequestPolicyTipOption {
    AllNodes,
    AllShards,
    MultiShard,
    Special,
}

impl TryFrom<&str> for RequestPolicyTipOption {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            "all_nodes" => Self::AllNodes,
            "all_shards" => Self::AllShards,
            "multi_shard" => Self::MultiShard,
            "special" => Self::Special,
            _ => return Err("not a valid value".into())
        })
    }
}


#[derive(Debug)]
pub enum ResponsePolicyTipOption {
    OneSucceeded,
    AllSucceeded,
    AggLogicalAnd,
    AggLogicalOr,
    AggMin,
    AggMax,
    AggSum,
    Special,
}

impl TryFrom<&str> for ResponsePolicyTipOption {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            "one_succeeded" => Self::OneSucceeded,
            "all_succeeded" => Self::AllSucceeded,
            "agg_logical_and" => Self::AggLogicalAnd,
            "agg_logical_or" => Self::AggLogicalOr,
            "agg_min" => Self::AggMin,
            "agg_max" => Self::AggMax,
            "agg_sum" => Self::AggSum,
            "special" => Self::Special,
            _ => return Err("not a valid value".into())
        })
    }
}



#[derive(Debug)]
pub enum CommandTip {
    NonDeterministicOutput,
    NonDeterministicOutputOrder,
    RequestPolicy(RequestPolicyTipOption),
    ResponsePolicy(ResponsePolicyTipOption),
}


impl TryFrom<String> for CommandTip {
    type Error = String;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "non_deterministic_output" => Self::NonDeterministicOutput,
            "non_deterministic_output_order" => Self::NonDeterministicOutputOrder,
            s if s.starts_with("request_policy:") => {
                let option = RequestPolicyTipOption::try_from(&s[15..])?;
                Self::RequestPolicy(option)
            },
            s if s.starts_with("response_policy:") => {
                let option = ResponsePolicyTipOption::try_from(&s[16..])?;
                Self::ResponsePolicy(option)
            }
            _ => return Err("not a valid value".into())
        })
    }
}



#[derive(Debug)]
pub enum Flag {
    Fast,
    Sentinel
}


impl Flag {
    pub fn to_string(&self) -> String {
        match self {
            Self::Fast => "fast",
            Self::Sentinel => "sentinel",
        }.to_string()
    }
}


impl TryFrom<String> for Flag {
    type Error = String;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "fast" => Self::Fast,
            "sentinel" => Self::Sentinel,
            _ => return Err("not a valid value".into())
        })
    }
}




#[derive(Debug)]
pub enum AclCategory {
    Connection,
}


impl AclCategory {
    pub fn to_string(&self) -> String {
        match self {
            Self::Connection => "@connection",
        }.to_string()
    }
}


impl TryFrom<String> for AclCategory {
    type Error = String;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "connection" => Self::Connection,
            _ => return Err("not a valid value".into())
        })
    }
}
