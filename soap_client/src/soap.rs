use std::io::{Read, Write};
use yaserde::ser::to_string;
use yaserde::{YaDeserialize, YaSerialize};

const SOAP_ENVELOPE: &str = "http://www.w3.org/2003/05/soap-envelope/";

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(
    rename = "Envelope",
    namespace = "soap: http://www.w3.org/2003/05/soap-envelope/"
)]
struct Envelope {
    #[yaserde(rename = "encodingStyle", prefix = "soap", attribute)]
    encoding_style: String,
    #[yaserde(rename = "tns", prefix = "xmlns")]
    tnsattr: String,
    #[yaserde(rename = "urn", prefix = "xmlns")]
    urnattr: String,
    #[yaserde(rename = "xsi", prefix = "xmlns")]
    xsiattr: String,
    #[yaserde(rename = "Header", prefix = "soap")]
    header: Message,
    #[yaserde(rename = "Body", prefix = "soap")]
    body: Message,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
struct Message {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope() {
        let e = Envelope {
            encoding_style: "http://www.w3.org/2003/05/soap-encoding".to_string(),
            tnsattr: "".to_string(),
            urnattr: "".to_string(),
            xsiattr: "".to_string(),
            header: Message {},
            body: Message {},
        };

        let xml = to_string(&e).expect("can not convert to XML");
        println!("{}", xml);
    }
}