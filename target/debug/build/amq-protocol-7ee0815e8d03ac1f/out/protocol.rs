use crate::types::*;
use crate::types::flags::*;
use crate::types::generation::*;
use crate::types::parsing::*;

use cookie_factory::{GenError, do_gen, gen_call, gen_cond};
use nom::*;

/// Protocol metadata
pub mod metadata {
    use super::*;

    /// The name of the protocol
    pub const NAME:          &str           = "AMQP";
    /// The major version of the protocol
    pub const MAJOR_VERSION: ShortShortUInt = 0;
    /// The minor version of the protocol
    pub const MINOR_VERSION: ShortShortUInt = 9;
    /// The revision (version) of the protocol
    pub const REVISION:      ShortShortUInt = 1;
    /// The default port of the protocol
    pub const PORT:          LongUInt       = 5672;
    /// The copyright holding the protocol
    pub const COPYRIGHT:     &str           = r#"Copyright (C) 2008-2016 Pivotal Software, Inc, Inc.

Permission is hereby granted, free of charge, to any person
obtaining a copy of this file (the "Software"), to deal in the
Software without restriction, including without limitation the 
rights to use, copy, modify, merge, publish, distribute, 
sublicense, and/or sell copies of the Software, and to permit 
persons to whom the Software is furnished to do so, subject to 
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Class information entered from amqp_xml0-8.pdf and domain types from amqp-xml-doc0-9.pdf
Updated for 0-9-1 by Tony Garnock-Jones

b3cb053f15e7b98808c0ccc67f23cb3e  amqp_xml0-8.pdf
http://www.twiststandards.org/index.php?option=com_docman&task=cat_view&gid=28&&Itemid=90
8444db91e2949dbecfb2585e9eef6d64  amqp-xml-doc0-9.pdf
https://jira.amqp.org/confluence/download/attachments/720900/amqp-xml-doc0-9.pdf?version=1
"#;
}

/// Protocol constants
pub mod constants {
    use super::*;

    /// FRAME-METHOD (Generated)
    pub const FRAME_METHOD: ShortShortUInt = 1;
    /// FRAME-HEADER (Generated)
    pub const FRAME_HEADER: ShortShortUInt = 2;
    /// FRAME-BODY (Generated)
    pub const FRAME_BODY: ShortShortUInt = 3;
    /// FRAME-HEARTBEAT (Generated)
    pub const FRAME_HEARTBEAT: ShortShortUInt = 8;
    /// FRAME-MIN-SIZE (Generated)
    pub const FRAME_MIN_SIZE: ShortUInt = 4096;
    /// FRAME-END (Generated)
    pub const FRAME_END: ShortShortUInt = 206;
    /// REPLY-SUCCESS (Generated)
    pub const REPLY_SUCCESS: ShortShortUInt = 200;
    }

/// An AMQP Error
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPError {
    /// A soft AMQP error
    Soft(AMQPSoftError),
    /// A hard AMQP error
    Hard(AMQPHardError),
}

impl AMQPError {
    /// Get the id of the error
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            AMQPError::Soft(ref s) => s.get_id(),
            AMQPError::Hard(ref h) => h.get_id(),
        }
    }

    /// Get the error corresponding to an id
    pub fn from_id(id: ShortUInt) -> Option<AMQPError> {
        AMQPSoftError::from_id(id).map(AMQPError::Soft).or_else(|| AMQPHardError::from_id(id).map(AMQPError::Hard))
    }
}

/// The available soft AMQP errors
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPSoftError {
    /// CONTENT-TOO-LARGE (Generated)
    CONTENTTOOLARGE,
    /// NO-ROUTE (Generated)
    NOROUTE,
    /// NO-CONSUMERS (Generated)
    NOCONSUMERS,
    /// ACCESS-REFUSED (Generated)
    ACCESSREFUSED,
    /// NOT-FOUND (Generated)
    NOTFOUND,
    /// RESOURCE-LOCKED (Generated)
    RESOURCELOCKED,
    /// PRECONDITION-FAILED (Generated)
    PRECONDITIONFAILED,
    }

impl AMQPSoftError {
    /// Get the id of the soft error
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            AMQPSoftError::CONTENTTOOLARGE => 311,
            AMQPSoftError::NOROUTE => 312,
            AMQPSoftError::NOCONSUMERS => 313,
            AMQPSoftError::ACCESSREFUSED => 403,
            AMQPSoftError::NOTFOUND => 404,
            AMQPSoftError::RESOURCELOCKED => 405,
            AMQPSoftError::PRECONDITIONFAILED => 406,
            }
    }

    /// Get the soft error corresponding to an id
    pub fn from_id(id: ShortUInt) -> Option<AMQPSoftError> {
        match id {
            311 => Some(AMQPSoftError::CONTENTTOOLARGE),
            312 => Some(AMQPSoftError::NOROUTE),
            313 => Some(AMQPSoftError::NOCONSUMERS),
            403 => Some(AMQPSoftError::ACCESSREFUSED),
            404 => Some(AMQPSoftError::NOTFOUND),
            405 => Some(AMQPSoftError::RESOURCELOCKED),
            406 => Some(AMQPSoftError::PRECONDITIONFAILED),
            _                  => None,
        }
    }
}

/// The available hard AMQP errors
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPHardError {
    /// CONNECTION-FORCED (Generated)
    CONNECTIONFORCED,
    /// INVALID-PATH (Generated)
    INVALIDPATH,
    /// FRAME-ERROR (Generated)
    FRAMEERROR,
    /// SYNTAX-ERROR (Generated)
    SYNTAXERROR,
    /// COMMAND-INVALID (Generated)
    COMMANDINVALID,
    /// CHANNEL-ERROR (Generated)
    CHANNELERROR,
    /// UNEXPECTED-FRAME (Generated)
    UNEXPECTEDFRAME,
    /// RESOURCE-ERROR (Generated)
    RESOURCEERROR,
    /// NOT-ALLOWED (Generated)
    NOTALLOWED,
    /// NOT-IMPLEMENTED (Generated)
    NOTIMPLEMENTED,
    /// INTERNAL-ERROR (Generated)
    INTERNALERROR,
    }

impl AMQPHardError {
    /// Get the id of the hard error
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            AMQPHardError::CONNECTIONFORCED => 320,
            AMQPHardError::INVALIDPATH => 402,
            AMQPHardError::FRAMEERROR => 501,
            AMQPHardError::SYNTAXERROR => 502,
            AMQPHardError::COMMANDINVALID => 503,
            AMQPHardError::CHANNELERROR => 504,
            AMQPHardError::UNEXPECTEDFRAME => 505,
            AMQPHardError::RESOURCEERROR => 506,
            AMQPHardError::NOTALLOWED => 530,
            AMQPHardError::NOTIMPLEMENTED => 540,
            AMQPHardError::INTERNALERROR => 541,
            }
    }

    /// Get the hard error corresponding to an id
    pub fn from_id(id: ShortUInt) -> Option<AMQPHardError> {
        match id {
            320 => Some(AMQPHardError::CONNECTIONFORCED),
            402 => Some(AMQPHardError::INVALIDPATH),
            501 => Some(AMQPHardError::FRAMEERROR),
            502 => Some(AMQPHardError::SYNTAXERROR),
            503 => Some(AMQPHardError::COMMANDINVALID),
            504 => Some(AMQPHardError::CHANNELERROR),
            505 => Some(AMQPHardError::UNEXPECTEDFRAME),
            506 => Some(AMQPHardError::RESOURCEERROR),
            530 => Some(AMQPHardError::NOTALLOWED),
            540 => Some(AMQPHardError::NOTIMPLEMENTED),
            541 => Some(AMQPHardError::INTERNALERROR),
            _                  => None,
        }
    }
}

use self::connection::parse_connection;
use self::channel::parse_channel;
use self::access::parse_access;
use self::exchange::parse_exchange;
use self::queue::parse_queue;
use self::basic::parse_basic;
use self::tx::parse_tx;
use self::confirm::parse_confirm;
named_attr!(#[doc =  "Parse an AMQP class"], pub parse_class<AMQPClass>, switch!(parse_id,
    10 => map!(call!(parse_connection), AMQPClass::Connection) |20 => map!(call!(parse_channel), AMQPClass::Channel) |30 => map!(call!(parse_access), AMQPClass::Access) |40 => map!(call!(parse_exchange), AMQPClass::Exchange) |50 => map!(call!(parse_queue), AMQPClass::Queue) |60 => map!(call!(parse_basic), AMQPClass::Basic) |90 => map!(call!(parse_tx), AMQPClass::Tx) |85 => map!(call!(parse_confirm), AMQPClass::Confirm) ));

/// Serialize an AMQP class
pub fn gen_class<'a>(input: (&'a mut [u8], usize), class: &AMQPClass) -> Result<(&'a mut [u8], usize), GenError> {
    match *class {
        AMQPClass::Connection(ref connection) => connection::gen_connection(input, connection),
        AMQPClass::Channel(ref channel) => channel::gen_channel(input, channel),
        AMQPClass::Access(ref access) => access::gen_access(input, access),
        AMQPClass::Exchange(ref exchange) => exchange::gen_exchange(input, exchange),
        AMQPClass::Queue(ref queue) => queue::gen_queue(input, queue),
        AMQPClass::Basic(ref basic) => basic::gen_basic(input, basic),
        AMQPClass::Tx(ref tx) => tx::gen_tx(input, tx),
        AMQPClass::Confirm(ref confirm) => confirm::gen_confirm(input, confirm),
        }
}

/// The available AMQP classes
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPClass {
    /// connection (Generated)
    Connection(connection::AMQPMethod),
    /// channel (Generated)
    Channel(channel::AMQPMethod),
    /// access (Generated)
    Access(access::AMQPMethod),
    /// exchange (Generated)
    Exchange(exchange::AMQPMethod),
    /// queue (Generated)
    Queue(queue::AMQPMethod),
    /// basic (Generated)
    Basic(basic::AMQPMethod),
    /// tx (Generated)
    Tx(tx::AMQPMethod),
    /// confirm (Generated)
    Confirm(confirm::AMQPMethod),
    }


/// connection (generated)
pub mod connection {
    use super::*;

    /// Get the id of the AMQP class
    pub fn get_id() -> ShortUInt {
        return 10;
    }

    named_attr!(#[doc = "Parse connection (Generated)"], pub parse_connection<connection::AMQPMethod>, switch!(parse_id,
        10 => map!(call!(parse_start), AMQPMethod::Start) |11 => map!(call!(parse_start_ok), AMQPMethod::StartOk) |20 => map!(call!(parse_secure), AMQPMethod::Secure) |21 => map!(call!(parse_secure_ok), AMQPMethod::SecureOk) |30 => map!(call!(parse_tune), AMQPMethod::Tune) |31 => map!(call!(parse_tune_ok), AMQPMethod::TuneOk) |40 => map!(call!(parse_open), AMQPMethod::Open) |41 => map!(call!(parse_open_ok), AMQPMethod::OpenOk) |50 => map!(call!(parse_close), AMQPMethod::Close) |51 => map!(call!(parse_close_ok), AMQPMethod::CloseOk) |60 => map!(call!(parse_blocked), AMQPMethod::Blocked) |61 => map!(call!(parse_unblocked), AMQPMethod::Unblocked) ));

    /// Serialize connection (Generated)
    pub fn gen_connection<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            AMQPMethod::Start(ref start) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_start(start)
                )
            },
            AMQPMethod::StartOk(ref start_ok) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_start_ok(start_ok)
                )
            },
            AMQPMethod::Secure(ref secure) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_secure(secure)
                )
            },
            AMQPMethod::SecureOk(ref secure_ok) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_secure_ok(secure_ok)
                )
            },
            AMQPMethod::Tune(ref tune) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_tune(tune)
                )
            },
            AMQPMethod::TuneOk(ref tune_ok) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_tune_ok(tune_ok)
                )
            },
            AMQPMethod::Open(ref open) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_open(open)
                )
            },
            AMQPMethod::OpenOk(ref open_ok) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_open_ok(open_ok)
                )
            },
            AMQPMethod::Close(ref close) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_close(close)
                )
            },
            AMQPMethod::CloseOk(ref close_ok) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_close_ok(close_ok)
                )
            },
            AMQPMethod::Blocked(ref blocked) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_blocked(blocked)
                )
            },
            AMQPMethod::Unblocked(ref unblocked) => {
                do_gen!(input,
                    gen_id(&10) >>
                    gen_unblocked(unblocked)
                )
            },
            }
    }

    /// The available methods in connection
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// start (Generated)
        Start(Start),
        /// start-ok (Generated)
        StartOk(StartOk),
        /// secure (Generated)
        Secure(Secure),
        /// secure-ok (Generated)
        SecureOk(SecureOk),
        /// tune (Generated)
        Tune(Tune),
        /// tune-ok (Generated)
        TuneOk(TuneOk),
        /// open (Generated)
        Open(Open),
        /// open-ok (Generated)
        OpenOk(OpenOk),
        /// close (Generated)
        Close(Close),
        /// close-ok (Generated)
        CloseOk(CloseOk),
        /// blocked (Generated)
        Blocked(Blocked),
        /// unblocked (Generated)
        Unblocked(Unblocked),
        }

    
    /// start (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Start {
        /// version-major (Generated)
        pub version_major: ShortShortUInt,
        /// version-minor (Generated)
        pub version_minor: ShortShortUInt,
        /// server-properties (Generated)
        pub server_properties: FieldTable,
        /// mechanisms (Generated)
        pub mechanisms: LongString,
        /// locales (Generated)
        pub locales: LongString,
        }

    impl Start {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 10;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse start (Generated)"], pub parse_start<Start>, do_parse!(
        version_major: parse_short_short_uint >>
        version_minor: parse_short_short_uint >>
        server_properties: parse_field_table >>
        mechanisms: parse_long_string >>
        locales: parse_long_string >>
        (Start {
            version_major,
            version_minor,
            server_properties,
            mechanisms,
            locales,
            })
    ));

    /// Serialize start (Generated)
    pub fn gen_start<'a>(input: (&'a mut [u8], usize), method: &Start) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&10)
            >> gen_short_short_uint(&method.version_major)
            >> gen_short_short_uint(&method.version_minor)
            >> gen_field_table(&method.server_properties)
            >> gen_long_string(&method.mechanisms)
            >> gen_long_string(&method.locales)
            )
    }
    
    /// start-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct StartOk {
        /// client-properties (Generated)
        pub client_properties: FieldTable,
        /// mechanism (Generated)
        pub mechanism: ShortString,
        /// response (Generated)
        pub response: LongString,
        /// locale (Generated)
        pub locale: ShortString,
        }

    impl StartOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 11;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse start-ok (Generated)"], pub parse_start_ok<StartOk>, do_parse!(
        client_properties: parse_field_table >>
        mechanism: parse_short_string >>
        response: parse_long_string >>
        locale: parse_short_string >>
        (StartOk {
            client_properties,
            mechanism,
            response,
            locale,
            })
    ));

    /// Serialize start-ok (Generated)
    pub fn gen_start_ok<'a>(input: (&'a mut [u8], usize), method: &StartOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&11)
            >> gen_field_table(&method.client_properties)
            >> gen_short_string(&method.mechanism)
            >> gen_long_string(&method.response)
            >> gen_short_string(&method.locale)
            )
    }
    
    /// secure (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Secure {
        /// challenge (Generated)
        pub challenge: LongString,
        }

    impl Secure {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 20;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse secure (Generated)"], pub parse_secure<Secure>, do_parse!(
        challenge: parse_long_string >>
        (Secure {
            challenge,
            })
    ));

    /// Serialize secure (Generated)
    pub fn gen_secure<'a>(input: (&'a mut [u8], usize), method: &Secure) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&20)
            >> gen_long_string(&method.challenge)
            )
    }
    
    /// secure-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct SecureOk {
        /// response (Generated)
        pub response: LongString,
        }

    impl SecureOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 21;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse secure-ok (Generated)"], pub parse_secure_ok<SecureOk>, do_parse!(
        response: parse_long_string >>
        (SecureOk {
            response,
            })
    ));

    /// Serialize secure-ok (Generated)
    pub fn gen_secure_ok<'a>(input: (&'a mut [u8], usize), method: &SecureOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&21)
            >> gen_long_string(&method.response)
            )
    }
    
    /// tune (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Tune {
        /// channel-max (Generated)
        pub channel_max: ShortUInt,
        /// frame-max (Generated)
        pub frame_max: LongUInt,
        /// heartbeat (Generated)
        pub heartbeat: ShortUInt,
        }

    impl Tune {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 30;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse tune (Generated)"], pub parse_tune<Tune>, do_parse!(
        channel_max: parse_short_uint >>
        frame_max: parse_long_uint >>
        heartbeat: parse_short_uint >>
        (Tune {
            channel_max,
            frame_max,
            heartbeat,
            })
    ));

    /// Serialize tune (Generated)
    pub fn gen_tune<'a>(input: (&'a mut [u8], usize), method: &Tune) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&30)
            >> gen_short_uint(&method.channel_max)
            >> gen_long_uint(&method.frame_max)
            >> gen_short_uint(&method.heartbeat)
            )
    }
    
    /// tune-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct TuneOk {
        /// channel-max (Generated)
        pub channel_max: ShortUInt,
        /// frame-max (Generated)
        pub frame_max: LongUInt,
        /// heartbeat (Generated)
        pub heartbeat: ShortUInt,
        }

    impl TuneOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 31;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse tune-ok (Generated)"], pub parse_tune_ok<TuneOk>, do_parse!(
        channel_max: parse_short_uint >>
        frame_max: parse_long_uint >>
        heartbeat: parse_short_uint >>
        (TuneOk {
            channel_max,
            frame_max,
            heartbeat,
            })
    ));

    /// Serialize tune-ok (Generated)
    pub fn gen_tune_ok<'a>(input: (&'a mut [u8], usize), method: &TuneOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&31)
            >> gen_short_uint(&method.channel_max)
            >> gen_long_uint(&method.frame_max)
            >> gen_short_uint(&method.heartbeat)
            )
    }
    
    /// open (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Open {
        /// virtual-host (Generated)
        pub virtual_host: ShortString,
        /// capabilities (Generated)
        pub capabilities: ShortString,
        
        /// insist (Generated)
        pub insist: Boolean,
        }

    impl Open {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 40;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse open (Generated)"], pub parse_open<Open>, do_parse!(
        virtual_host: parse_short_string >>
        capabilities: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "insist",
            ]) >>
        (Open {
            virtual_host,
            capabilities,
            
            insist: flags.get_flag("insist").unwrap_or(false),
            })
    ));

    /// Serialize open (Generated)
    pub fn gen_open<'a>(input: (&'a mut [u8], usize), method: &Open) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("insist".to_string(), method.insist);
        do_gen!(input,
            gen_id(&40)
            >> gen_short_string(&method.virtual_host)
            >> gen_short_string(&method.capabilities)
            
            >> gen_flags(&flags)
            )
    }
    
    /// open-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct OpenOk {
        /// known-hosts (Generated)
        pub known_hosts: ShortString,
        }

    impl OpenOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 41;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse open-ok (Generated)"], pub parse_open_ok<OpenOk>, do_parse!(
        known_hosts: parse_short_string >>
        (OpenOk {
            known_hosts,
            })
    ));

    /// Serialize open-ok (Generated)
    pub fn gen_open_ok<'a>(input: (&'a mut [u8], usize), method: &OpenOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&41)
            >> gen_short_string(&method.known_hosts)
            )
    }
    
    /// close (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Close {
        /// reply-code (Generated)
        pub reply_code: ShortUInt,
        /// reply-text (Generated)
        pub reply_text: ShortString,
        /// class-id (Generated)
        pub class_id: ShortUInt,
        /// method-id (Generated)
        pub method_id: ShortUInt,
        }

    impl Close {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 50;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse close (Generated)"], pub parse_close<Close>, do_parse!(
        reply_code: parse_short_uint >>
        reply_text: parse_short_string >>
        class_id: parse_short_uint >>
        method_id: parse_short_uint >>
        (Close {
            reply_code,
            reply_text,
            class_id,
            method_id,
            })
    ));

    /// Serialize close (Generated)
    pub fn gen_close<'a>(input: (&'a mut [u8], usize), method: &Close) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&50)
            >> gen_short_uint(&method.reply_code)
            >> gen_short_string(&method.reply_text)
            >> gen_short_uint(&method.class_id)
            >> gen_short_uint(&method.method_id)
            )
    }
    
    /// close-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct CloseOk {
        }

    impl CloseOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 51;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse close-ok (Generated)"], pub parse_close_ok<CloseOk>, do_parse!(
        (CloseOk {
            })
    ));

    /// Serialize close-ok (Generated)
    pub fn gen_close_ok<'a>(input: (&'a mut [u8], usize), _: &CloseOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&51)
            )
    }
    
    /// blocked (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Blocked {
        /// reason (Generated)
        pub reason: ShortString,
        }

    impl Blocked {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 60;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse blocked (Generated)"], pub parse_blocked<Blocked>, do_parse!(
        reason: parse_short_string >>
        (Blocked {
            reason,
            })
    ));

    /// Serialize blocked (Generated)
    pub fn gen_blocked<'a>(input: (&'a mut [u8], usize), method: &Blocked) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&60)
            >> gen_short_string(&method.reason)
            )
    }
    
    /// unblocked (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Unblocked {
        }

    impl Unblocked {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 61;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 10;
        }
    }

    named_attr!(#[doc = "Parse unblocked (Generated)"], pub parse_unblocked<Unblocked>, do_parse!(
        (Unblocked {
            })
    ));

    /// Serialize unblocked (Generated)
    pub fn gen_unblocked<'a>(input: (&'a mut [u8], usize), _: &Unblocked) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&61)
            )
    }
    }

/// channel (generated)
pub mod channel {
    use super::*;

    /// Get the id of the AMQP class
    pub fn get_id() -> ShortUInt {
        return 20;
    }

    named_attr!(#[doc = "Parse channel (Generated)"], pub parse_channel<channel::AMQPMethod>, switch!(parse_id,
        10 => map!(call!(parse_open), AMQPMethod::Open) |11 => map!(call!(parse_open_ok), AMQPMethod::OpenOk) |20 => map!(call!(parse_flow), AMQPMethod::Flow) |21 => map!(call!(parse_flow_ok), AMQPMethod::FlowOk) |40 => map!(call!(parse_close), AMQPMethod::Close) |41 => map!(call!(parse_close_ok), AMQPMethod::CloseOk) ));

    /// Serialize channel (Generated)
    pub fn gen_channel<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            AMQPMethod::Open(ref open) => {
                do_gen!(input,
                    gen_id(&20) >>
                    gen_open(open)
                )
            },
            AMQPMethod::OpenOk(ref open_ok) => {
                do_gen!(input,
                    gen_id(&20) >>
                    gen_open_ok(open_ok)
                )
            },
            AMQPMethod::Flow(ref flow) => {
                do_gen!(input,
                    gen_id(&20) >>
                    gen_flow(flow)
                )
            },
            AMQPMethod::FlowOk(ref flow_ok) => {
                do_gen!(input,
                    gen_id(&20) >>
                    gen_flow_ok(flow_ok)
                )
            },
            AMQPMethod::Close(ref close) => {
                do_gen!(input,
                    gen_id(&20) >>
                    gen_close(close)
                )
            },
            AMQPMethod::CloseOk(ref close_ok) => {
                do_gen!(input,
                    gen_id(&20) >>
                    gen_close_ok(close_ok)
                )
            },
            }
    }

    /// The available methods in channel
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// open (Generated)
        Open(Open),
        /// open-ok (Generated)
        OpenOk(OpenOk),
        /// flow (Generated)
        Flow(Flow),
        /// flow-ok (Generated)
        FlowOk(FlowOk),
        /// close (Generated)
        Close(Close),
        /// close-ok (Generated)
        CloseOk(CloseOk),
        }

    
    /// open (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Open {
        /// out-of-band (Generated)
        pub out_of_band: ShortString,
        }

    impl Open {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 10;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 20;
        }
    }

    named_attr!(#[doc = "Parse open (Generated)"], pub parse_open<Open>, do_parse!(
        out_of_band: parse_short_string >>
        (Open {
            out_of_band,
            })
    ));

    /// Serialize open (Generated)
    pub fn gen_open<'a>(input: (&'a mut [u8], usize), method: &Open) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&10)
            >> gen_short_string(&method.out_of_band)
            )
    }
    
    /// open-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct OpenOk {
        /// channel-id (Generated)
        pub channel_id: LongString,
        }

    impl OpenOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 11;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 20;
        }
    }

    named_attr!(#[doc = "Parse open-ok (Generated)"], pub parse_open_ok<OpenOk>, do_parse!(
        channel_id: parse_long_string >>
        (OpenOk {
            channel_id,
            })
    ));

    /// Serialize open-ok (Generated)
    pub fn gen_open_ok<'a>(input: (&'a mut [u8], usize), method: &OpenOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&11)
            >> gen_long_string(&method.channel_id)
            )
    }
    
    /// flow (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Flow {
        
        /// active (Generated)
        pub active: Boolean,
        }

    impl Flow {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 20;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 20;
        }
    }

    named_attr!(#[doc = "Parse flow (Generated)"], pub parse_flow<Flow>, do_parse!(
        
        flags: apply!(parse_flags, &[
            "active",
            ]) >>
        (Flow {
            
            active: flags.get_flag("active").unwrap_or(false),
            })
    ));

    /// Serialize flow (Generated)
    pub fn gen_flow<'a>(input: (&'a mut [u8], usize), method: &Flow) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("active".to_string(), method.active);
        do_gen!(input,
            gen_id(&20)
            
            >> gen_flags(&flags)
            )
    }
    
    /// flow-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct FlowOk {
        
        /// active (Generated)
        pub active: Boolean,
        }

    impl FlowOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 21;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 20;
        }
    }

    named_attr!(#[doc = "Parse flow-ok (Generated)"], pub parse_flow_ok<FlowOk>, do_parse!(
        
        flags: apply!(parse_flags, &[
            "active",
            ]) >>
        (FlowOk {
            
            active: flags.get_flag("active").unwrap_or(false),
            })
    ));

    /// Serialize flow-ok (Generated)
    pub fn gen_flow_ok<'a>(input: (&'a mut [u8], usize), method: &FlowOk) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("active".to_string(), method.active);
        do_gen!(input,
            gen_id(&21)
            
            >> gen_flags(&flags)
            )
    }
    
    /// close (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Close {
        /// reply-code (Generated)
        pub reply_code: ShortUInt,
        /// reply-text (Generated)
        pub reply_text: ShortString,
        /// class-id (Generated)
        pub class_id: ShortUInt,
        /// method-id (Generated)
        pub method_id: ShortUInt,
        }

    impl Close {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 40;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 20;
        }
    }

    named_attr!(#[doc = "Parse close (Generated)"], pub parse_close<Close>, do_parse!(
        reply_code: parse_short_uint >>
        reply_text: parse_short_string >>
        class_id: parse_short_uint >>
        method_id: parse_short_uint >>
        (Close {
            reply_code,
            reply_text,
            class_id,
            method_id,
            })
    ));

    /// Serialize close (Generated)
    pub fn gen_close<'a>(input: (&'a mut [u8], usize), method: &Close) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&40)
            >> gen_short_uint(&method.reply_code)
            >> gen_short_string(&method.reply_text)
            >> gen_short_uint(&method.class_id)
            >> gen_short_uint(&method.method_id)
            )
    }
    
    /// close-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct CloseOk {
        }

    impl CloseOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 41;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 20;
        }
    }

    named_attr!(#[doc = "Parse close-ok (Generated)"], pub parse_close_ok<CloseOk>, do_parse!(
        (CloseOk {
            })
    ));

    /// Serialize close-ok (Generated)
    pub fn gen_close_ok<'a>(input: (&'a mut [u8], usize), _: &CloseOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&41)
            )
    }
    }

/// access (generated)
pub mod access {
    use super::*;

    /// Get the id of the AMQP class
    pub fn get_id() -> ShortUInt {
        return 30;
    }

    named_attr!(#[doc = "Parse access (Generated)"], pub parse_access<access::AMQPMethod>, switch!(parse_id,
        10 => map!(call!(parse_request), AMQPMethod::Request) |11 => map!(call!(parse_request_ok), AMQPMethod::RequestOk) ));

    /// Serialize access (Generated)
    pub fn gen_access<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            AMQPMethod::Request(ref request) => {
                do_gen!(input,
                    gen_id(&30) >>
                    gen_request(request)
                )
            },
            AMQPMethod::RequestOk(ref request_ok) => {
                do_gen!(input,
                    gen_id(&30) >>
                    gen_request_ok(request_ok)
                )
            },
            }
    }

    /// The available methods in access
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// request (Generated)
        Request(Request),
        /// request-ok (Generated)
        RequestOk(RequestOk),
        }

    
    /// request (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Request {
        /// realm (Generated)
        pub realm: ShortString,
        
        /// exclusive (Generated)
        pub exclusive: Boolean,
        /// passive (Generated)
        pub passive: Boolean,
        /// active (Generated)
        pub active: Boolean,
        /// write (Generated)
        pub write: Boolean,
        /// read (Generated)
        pub read: Boolean,
        }

    impl Request {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 10;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 30;
        }
    }

    named_attr!(#[doc = "Parse request (Generated)"], pub parse_request<Request>, do_parse!(
        realm: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "exclusive",
            "passive",
            "active",
            "write",
            "read",
            ]) >>
        (Request {
            realm,
            
            exclusive: flags.get_flag("exclusive").unwrap_or(false),
            passive: flags.get_flag("passive").unwrap_or(false),
            active: flags.get_flag("active").unwrap_or(false),
            write: flags.get_flag("write").unwrap_or(false),
            read: flags.get_flag("read").unwrap_or(false),
            })
    ));

    /// Serialize request (Generated)
    pub fn gen_request<'a>(input: (&'a mut [u8], usize), method: &Request) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("exclusive".to_string(), method.exclusive);
        flags.add_flag("passive".to_string(), method.passive);
        flags.add_flag("active".to_string(), method.active);
        flags.add_flag("write".to_string(), method.write);
        flags.add_flag("read".to_string(), method.read);
        do_gen!(input,
            gen_id(&10)
            >> gen_short_string(&method.realm)
            
            >> gen_flags(&flags)
            )
    }
    
    /// request-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct RequestOk {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        }

    impl RequestOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 11;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 30;
        }
    }

    named_attr!(#[doc = "Parse request-ok (Generated)"], pub parse_request_ok<RequestOk>, do_parse!(
        ticket: parse_short_uint >>
        (RequestOk {
            ticket,
            })
    ));

    /// Serialize request-ok (Generated)
    pub fn gen_request_ok<'a>(input: (&'a mut [u8], usize), method: &RequestOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&11)
            >> gen_short_uint(&method.ticket)
            )
    }
    }

/// exchange (generated)
pub mod exchange {
    use super::*;

    /// Get the id of the AMQP class
    pub fn get_id() -> ShortUInt {
        return 40;
    }

    named_attr!(#[doc = "Parse exchange (Generated)"], pub parse_exchange<exchange::AMQPMethod>, switch!(parse_id,
        10 => map!(call!(parse_declare), AMQPMethod::Declare) |11 => map!(call!(parse_declare_ok), AMQPMethod::DeclareOk) |20 => map!(call!(parse_delete), AMQPMethod::Delete) |21 => map!(call!(parse_delete_ok), AMQPMethod::DeleteOk) |30 => map!(call!(parse_bind), AMQPMethod::Bind) |31 => map!(call!(parse_bind_ok), AMQPMethod::BindOk) |40 => map!(call!(parse_unbind), AMQPMethod::Unbind) |51 => map!(call!(parse_unbind_ok), AMQPMethod::UnbindOk) ));

    /// Serialize exchange (Generated)
    pub fn gen_exchange<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            AMQPMethod::Declare(ref declare) => {
                do_gen!(input,
                    gen_id(&40) >>
                    gen_declare(declare)
                )
            },
            AMQPMethod::DeclareOk(ref declare_ok) => {
                do_gen!(input,
                    gen_id(&40) >>
                    gen_declare_ok(declare_ok)
                )
            },
            AMQPMethod::Delete(ref delete) => {
                do_gen!(input,
                    gen_id(&40) >>
                    gen_delete(delete)
                )
            },
            AMQPMethod::DeleteOk(ref delete_ok) => {
                do_gen!(input,
                    gen_id(&40) >>
                    gen_delete_ok(delete_ok)
                )
            },
            AMQPMethod::Bind(ref bind) => {
                do_gen!(input,
                    gen_id(&40) >>
                    gen_bind(bind)
                )
            },
            AMQPMethod::BindOk(ref bind_ok) => {
                do_gen!(input,
                    gen_id(&40) >>
                    gen_bind_ok(bind_ok)
                )
            },
            AMQPMethod::Unbind(ref unbind) => {
                do_gen!(input,
                    gen_id(&40) >>
                    gen_unbind(unbind)
                )
            },
            AMQPMethod::UnbindOk(ref unbind_ok) => {
                do_gen!(input,
                    gen_id(&40) >>
                    gen_unbind_ok(unbind_ok)
                )
            },
            }
    }

    /// The available methods in exchange
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// declare (Generated)
        Declare(Declare),
        /// declare-ok (Generated)
        DeclareOk(DeclareOk),
        /// delete (Generated)
        Delete(Delete),
        /// delete-ok (Generated)
        DeleteOk(DeleteOk),
        /// bind (Generated)
        Bind(Bind),
        /// bind-ok (Generated)
        BindOk(BindOk),
        /// unbind (Generated)
        Unbind(Unbind),
        /// unbind-ok (Generated)
        UnbindOk(UnbindOk),
        }

    
    /// declare (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Declare {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// type (Generated)
        pub type_: ShortString,
        
        /// passive (Generated)
        pub passive: Boolean,
        /// durable (Generated)
        pub durable: Boolean,
        /// auto-delete (Generated)
        pub auto_delete: Boolean,
        /// internal (Generated)
        pub internal: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
        }

    impl Declare {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 10;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 40;
        }
    }

    named_attr!(#[doc = "Parse declare (Generated)"], pub parse_declare<Declare>, do_parse!(
        ticket: parse_short_uint >>
        exchange: parse_short_string >>
        type_: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "passive",
            "durable",
            "auto-delete",
            "internal",
            "nowait",
            ]) >>
        arguments: parse_field_table >>
        (Declare {
            ticket,
            exchange,
            type_,
            
            passive: flags.get_flag("passive").unwrap_or(false),
            durable: flags.get_flag("durable").unwrap_or(false),
            auto_delete: flags.get_flag("auto_delete").unwrap_or(false),
            internal: flags.get_flag("internal").unwrap_or(false),
            nowait: flags.get_flag("nowait").unwrap_or(false),
            arguments,
            })
    ));

    /// Serialize declare (Generated)
    pub fn gen_declare<'a>(input: (&'a mut [u8], usize), method: &Declare) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("passive".to_string(), method.passive);
        flags.add_flag("durable".to_string(), method.durable);
        flags.add_flag("auto_delete".to_string(), method.auto_delete);
        flags.add_flag("internal".to_string(), method.internal);
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&10)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.exchange)
            >> gen_short_string(&method.type_)
            
            >> gen_flags(&flags)
            >> gen_field_table(&method.arguments)
            )
    }
    
    /// declare-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct DeclareOk {
        }

    impl DeclareOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 11;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 40;
        }
    }

    named_attr!(#[doc = "Parse declare-ok (Generated)"], pub parse_declare_ok<DeclareOk>, do_parse!(
        (DeclareOk {
            })
    ));

    /// Serialize declare-ok (Generated)
    pub fn gen_declare_ok<'a>(input: (&'a mut [u8], usize), _: &DeclareOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&11)
            )
    }
    
    /// delete (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Delete {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// exchange (Generated)
        pub exchange: ShortString,
        
        /// if-unused (Generated)
        pub if_unused: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
        }

    impl Delete {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 20;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 40;
        }
    }

    named_attr!(#[doc = "Parse delete (Generated)"], pub parse_delete<Delete>, do_parse!(
        ticket: parse_short_uint >>
        exchange: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "if-unused",
            "nowait",
            ]) >>
        (Delete {
            ticket,
            exchange,
            
            if_unused: flags.get_flag("if_unused").unwrap_or(false),
            nowait: flags.get_flag("nowait").unwrap_or(false),
            })
    ));

    /// Serialize delete (Generated)
    pub fn gen_delete<'a>(input: (&'a mut [u8], usize), method: &Delete) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("if_unused".to_string(), method.if_unused);
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&20)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.exchange)
            
            >> gen_flags(&flags)
            )
    }
    
    /// delete-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct DeleteOk {
        }

    impl DeleteOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 21;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 40;
        }
    }

    named_attr!(#[doc = "Parse delete-ok (Generated)"], pub parse_delete_ok<DeleteOk>, do_parse!(
        (DeleteOk {
            })
    ));

    /// Serialize delete-ok (Generated)
    pub fn gen_delete_ok<'a>(input: (&'a mut [u8], usize), _: &DeleteOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&21)
            )
    }
    
    /// bind (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Bind {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// destination (Generated)
        pub destination: ShortString,
        /// source (Generated)
        pub source: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
        }

    impl Bind {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 30;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 40;
        }
    }

    named_attr!(#[doc = "Parse bind (Generated)"], pub parse_bind<Bind>, do_parse!(
        ticket: parse_short_uint >>
        destination: parse_short_string >>
        source: parse_short_string >>
        routing_key: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "nowait",
            ]) >>
        arguments: parse_field_table >>
        (Bind {
            ticket,
            destination,
            source,
            routing_key,
            
            nowait: flags.get_flag("nowait").unwrap_or(false),
            arguments,
            })
    ));

    /// Serialize bind (Generated)
    pub fn gen_bind<'a>(input: (&'a mut [u8], usize), method: &Bind) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&30)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.destination)
            >> gen_short_string(&method.source)
            >> gen_short_string(&method.routing_key)
            
            >> gen_flags(&flags)
            >> gen_field_table(&method.arguments)
            )
    }
    
    /// bind-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct BindOk {
        }

    impl BindOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 31;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 40;
        }
    }

    named_attr!(#[doc = "Parse bind-ok (Generated)"], pub parse_bind_ok<BindOk>, do_parse!(
        (BindOk {
            })
    ));

    /// Serialize bind-ok (Generated)
    pub fn gen_bind_ok<'a>(input: (&'a mut [u8], usize), _: &BindOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&31)
            )
    }
    
    /// unbind (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Unbind {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// destination (Generated)
        pub destination: ShortString,
        /// source (Generated)
        pub source: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
        }

    impl Unbind {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 40;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 40;
        }
    }

    named_attr!(#[doc = "Parse unbind (Generated)"], pub parse_unbind<Unbind>, do_parse!(
        ticket: parse_short_uint >>
        destination: parse_short_string >>
        source: parse_short_string >>
        routing_key: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "nowait",
            ]) >>
        arguments: parse_field_table >>
        (Unbind {
            ticket,
            destination,
            source,
            routing_key,
            
            nowait: flags.get_flag("nowait").unwrap_or(false),
            arguments,
            })
    ));

    /// Serialize unbind (Generated)
    pub fn gen_unbind<'a>(input: (&'a mut [u8], usize), method: &Unbind) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&40)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.destination)
            >> gen_short_string(&method.source)
            >> gen_short_string(&method.routing_key)
            
            >> gen_flags(&flags)
            >> gen_field_table(&method.arguments)
            )
    }
    
    /// unbind-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct UnbindOk {
        }

    impl UnbindOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 51;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 40;
        }
    }

    named_attr!(#[doc = "Parse unbind-ok (Generated)"], pub parse_unbind_ok<UnbindOk>, do_parse!(
        (UnbindOk {
            })
    ));

    /// Serialize unbind-ok (Generated)
    pub fn gen_unbind_ok<'a>(input: (&'a mut [u8], usize), _: &UnbindOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&51)
            )
    }
    }

/// queue (generated)
pub mod queue {
    use super::*;

    /// Get the id of the AMQP class
    pub fn get_id() -> ShortUInt {
        return 50;
    }

    named_attr!(#[doc = "Parse queue (Generated)"], pub parse_queue<queue::AMQPMethod>, switch!(parse_id,
        10 => map!(call!(parse_declare), AMQPMethod::Declare) |11 => map!(call!(parse_declare_ok), AMQPMethod::DeclareOk) |20 => map!(call!(parse_bind), AMQPMethod::Bind) |21 => map!(call!(parse_bind_ok), AMQPMethod::BindOk) |30 => map!(call!(parse_purge), AMQPMethod::Purge) |31 => map!(call!(parse_purge_ok), AMQPMethod::PurgeOk) |40 => map!(call!(parse_delete), AMQPMethod::Delete) |41 => map!(call!(parse_delete_ok), AMQPMethod::DeleteOk) |50 => map!(call!(parse_unbind), AMQPMethod::Unbind) |51 => map!(call!(parse_unbind_ok), AMQPMethod::UnbindOk) ));

    /// Serialize queue (Generated)
    pub fn gen_queue<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            AMQPMethod::Declare(ref declare) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_declare(declare)
                )
            },
            AMQPMethod::DeclareOk(ref declare_ok) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_declare_ok(declare_ok)
                )
            },
            AMQPMethod::Bind(ref bind) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_bind(bind)
                )
            },
            AMQPMethod::BindOk(ref bind_ok) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_bind_ok(bind_ok)
                )
            },
            AMQPMethod::Purge(ref purge) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_purge(purge)
                )
            },
            AMQPMethod::PurgeOk(ref purge_ok) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_purge_ok(purge_ok)
                )
            },
            AMQPMethod::Delete(ref delete) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_delete(delete)
                )
            },
            AMQPMethod::DeleteOk(ref delete_ok) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_delete_ok(delete_ok)
                )
            },
            AMQPMethod::Unbind(ref unbind) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_unbind(unbind)
                )
            },
            AMQPMethod::UnbindOk(ref unbind_ok) => {
                do_gen!(input,
                    gen_id(&50) >>
                    gen_unbind_ok(unbind_ok)
                )
            },
            }
    }

    /// The available methods in queue
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// declare (Generated)
        Declare(Declare),
        /// declare-ok (Generated)
        DeclareOk(DeclareOk),
        /// bind (Generated)
        Bind(Bind),
        /// bind-ok (Generated)
        BindOk(BindOk),
        /// purge (Generated)
        Purge(Purge),
        /// purge-ok (Generated)
        PurgeOk(PurgeOk),
        /// delete (Generated)
        Delete(Delete),
        /// delete-ok (Generated)
        DeleteOk(DeleteOk),
        /// unbind (Generated)
        Unbind(Unbind),
        /// unbind-ok (Generated)
        UnbindOk(UnbindOk),
        }

    
    /// declare (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Declare {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// queue (Generated)
        pub queue: ShortString,
        
        /// passive (Generated)
        pub passive: Boolean,
        /// durable (Generated)
        pub durable: Boolean,
        /// exclusive (Generated)
        pub exclusive: Boolean,
        /// auto-delete (Generated)
        pub auto_delete: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
        }

    impl Declare {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 10;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse declare (Generated)"], pub parse_declare<Declare>, do_parse!(
        ticket: parse_short_uint >>
        queue: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "passive",
            "durable",
            "exclusive",
            "auto-delete",
            "nowait",
            ]) >>
        arguments: parse_field_table >>
        (Declare {
            ticket,
            queue,
            
            passive: flags.get_flag("passive").unwrap_or(false),
            durable: flags.get_flag("durable").unwrap_or(false),
            exclusive: flags.get_flag("exclusive").unwrap_or(false),
            auto_delete: flags.get_flag("auto_delete").unwrap_or(false),
            nowait: flags.get_flag("nowait").unwrap_or(false),
            arguments,
            })
    ));

    /// Serialize declare (Generated)
    pub fn gen_declare<'a>(input: (&'a mut [u8], usize), method: &Declare) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("passive".to_string(), method.passive);
        flags.add_flag("durable".to_string(), method.durable);
        flags.add_flag("exclusive".to_string(), method.exclusive);
        flags.add_flag("auto_delete".to_string(), method.auto_delete);
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&10)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.queue)
            
            >> gen_flags(&flags)
            >> gen_field_table(&method.arguments)
            )
    }
    
    /// declare-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct DeclareOk {
        /// queue (Generated)
        pub queue: ShortString,
        /// message-count (Generated)
        pub message_count: LongUInt,
        /// consumer-count (Generated)
        pub consumer_count: LongUInt,
        }

    impl DeclareOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 11;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse declare-ok (Generated)"], pub parse_declare_ok<DeclareOk>, do_parse!(
        queue: parse_short_string >>
        message_count: parse_long_uint >>
        consumer_count: parse_long_uint >>
        (DeclareOk {
            queue,
            message_count,
            consumer_count,
            })
    ));

    /// Serialize declare-ok (Generated)
    pub fn gen_declare_ok<'a>(input: (&'a mut [u8], usize), method: &DeclareOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&11)
            >> gen_short_string(&method.queue)
            >> gen_long_uint(&method.message_count)
            >> gen_long_uint(&method.consumer_count)
            )
    }
    
    /// bind (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Bind {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// queue (Generated)
        pub queue: ShortString,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
        }

    impl Bind {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 20;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse bind (Generated)"], pub parse_bind<Bind>, do_parse!(
        ticket: parse_short_uint >>
        queue: parse_short_string >>
        exchange: parse_short_string >>
        routing_key: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "nowait",
            ]) >>
        arguments: parse_field_table >>
        (Bind {
            ticket,
            queue,
            exchange,
            routing_key,
            
            nowait: flags.get_flag("nowait").unwrap_or(false),
            arguments,
            })
    ));

    /// Serialize bind (Generated)
    pub fn gen_bind<'a>(input: (&'a mut [u8], usize), method: &Bind) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&20)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.queue)
            >> gen_short_string(&method.exchange)
            >> gen_short_string(&method.routing_key)
            
            >> gen_flags(&flags)
            >> gen_field_table(&method.arguments)
            )
    }
    
    /// bind-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct BindOk {
        }

    impl BindOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 21;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse bind-ok (Generated)"], pub parse_bind_ok<BindOk>, do_parse!(
        (BindOk {
            })
    ));

    /// Serialize bind-ok (Generated)
    pub fn gen_bind_ok<'a>(input: (&'a mut [u8], usize), _: &BindOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&21)
            )
    }
    
    /// purge (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Purge {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// queue (Generated)
        pub queue: ShortString,
        
        /// nowait (Generated)
        pub nowait: Boolean,
        }

    impl Purge {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 30;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse purge (Generated)"], pub parse_purge<Purge>, do_parse!(
        ticket: parse_short_uint >>
        queue: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "nowait",
            ]) >>
        (Purge {
            ticket,
            queue,
            
            nowait: flags.get_flag("nowait").unwrap_or(false),
            })
    ));

    /// Serialize purge (Generated)
    pub fn gen_purge<'a>(input: (&'a mut [u8], usize), method: &Purge) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&30)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.queue)
            
            >> gen_flags(&flags)
            )
    }
    
    /// purge-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct PurgeOk {
        /// message-count (Generated)
        pub message_count: LongUInt,
        }

    impl PurgeOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 31;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse purge-ok (Generated)"], pub parse_purge_ok<PurgeOk>, do_parse!(
        message_count: parse_long_uint >>
        (PurgeOk {
            message_count,
            })
    ));

    /// Serialize purge-ok (Generated)
    pub fn gen_purge_ok<'a>(input: (&'a mut [u8], usize), method: &PurgeOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&31)
            >> gen_long_uint(&method.message_count)
            )
    }
    
    /// delete (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Delete {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// queue (Generated)
        pub queue: ShortString,
        
        /// if-unused (Generated)
        pub if_unused: Boolean,
        /// if-empty (Generated)
        pub if_empty: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
        }

    impl Delete {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 40;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse delete (Generated)"], pub parse_delete<Delete>, do_parse!(
        ticket: parse_short_uint >>
        queue: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "if-unused",
            "if-empty",
            "nowait",
            ]) >>
        (Delete {
            ticket,
            queue,
            
            if_unused: flags.get_flag("if_unused").unwrap_or(false),
            if_empty: flags.get_flag("if_empty").unwrap_or(false),
            nowait: flags.get_flag("nowait").unwrap_or(false),
            })
    ));

    /// Serialize delete (Generated)
    pub fn gen_delete<'a>(input: (&'a mut [u8], usize), method: &Delete) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("if_unused".to_string(), method.if_unused);
        flags.add_flag("if_empty".to_string(), method.if_empty);
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&40)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.queue)
            
            >> gen_flags(&flags)
            )
    }
    
    /// delete-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct DeleteOk {
        /// message-count (Generated)
        pub message_count: LongUInt,
        }

    impl DeleteOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 41;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse delete-ok (Generated)"], pub parse_delete_ok<DeleteOk>, do_parse!(
        message_count: parse_long_uint >>
        (DeleteOk {
            message_count,
            })
    ));

    /// Serialize delete-ok (Generated)
    pub fn gen_delete_ok<'a>(input: (&'a mut [u8], usize), method: &DeleteOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&41)
            >> gen_long_uint(&method.message_count)
            )
    }
    
    /// unbind (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Unbind {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// queue (Generated)
        pub queue: ShortString,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        /// arguments (Generated)
        pub arguments: FieldTable,
        }

    impl Unbind {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 50;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse unbind (Generated)"], pub parse_unbind<Unbind>, do_parse!(
        ticket: parse_short_uint >>
        queue: parse_short_string >>
        exchange: parse_short_string >>
        routing_key: parse_short_string >>
        arguments: parse_field_table >>
        (Unbind {
            ticket,
            queue,
            exchange,
            routing_key,
            arguments,
            })
    ));

    /// Serialize unbind (Generated)
    pub fn gen_unbind<'a>(input: (&'a mut [u8], usize), method: &Unbind) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&50)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.queue)
            >> gen_short_string(&method.exchange)
            >> gen_short_string(&method.routing_key)
            >> gen_field_table(&method.arguments)
            )
    }
    
    /// unbind-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct UnbindOk {
        }

    impl UnbindOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 51;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 50;
        }
    }

    named_attr!(#[doc = "Parse unbind-ok (Generated)"], pub parse_unbind_ok<UnbindOk>, do_parse!(
        (UnbindOk {
            })
    ));

    /// Serialize unbind-ok (Generated)
    pub fn gen_unbind_ok<'a>(input: (&'a mut [u8], usize), _: &UnbindOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&51)
            )
    }
    }

/// basic (generated)
pub mod basic {
    use super::*;

    /// Get the id of the AMQP class
    pub fn get_id() -> ShortUInt {
        return 60;
    }

    named_attr!(#[doc = "Parse basic (Generated)"], pub parse_basic<basic::AMQPMethod>, switch!(parse_id,
        10 => map!(call!(parse_qos), AMQPMethod::Qos) |11 => map!(call!(parse_qos_ok), AMQPMethod::QosOk) |20 => map!(call!(parse_consume), AMQPMethod::Consume) |21 => map!(call!(parse_consume_ok), AMQPMethod::ConsumeOk) |30 => map!(call!(parse_cancel), AMQPMethod::Cancel) |31 => map!(call!(parse_cancel_ok), AMQPMethod::CancelOk) |40 => map!(call!(parse_publish), AMQPMethod::Publish) |50 => map!(call!(parse_return_), AMQPMethod::Return) |60 => map!(call!(parse_deliver), AMQPMethod::Deliver) |70 => map!(call!(parse_get), AMQPMethod::Get) |71 => map!(call!(parse_get_ok), AMQPMethod::GetOk) |72 => map!(call!(parse_get_empty), AMQPMethod::GetEmpty) |80 => map!(call!(parse_ack), AMQPMethod::Ack) |90 => map!(call!(parse_reject), AMQPMethod::Reject) |100 => map!(call!(parse_recover_async), AMQPMethod::RecoverAsync) |110 => map!(call!(parse_recover), AMQPMethod::Recover) |111 => map!(call!(parse_recover_ok), AMQPMethod::RecoverOk) |120 => map!(call!(parse_nack), AMQPMethod::Nack) ));

    /// Serialize basic (Generated)
    pub fn gen_basic<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            AMQPMethod::Qos(ref qos) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_qos(qos)
                )
            },
            AMQPMethod::QosOk(ref qos_ok) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_qos_ok(qos_ok)
                )
            },
            AMQPMethod::Consume(ref consume) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_consume(consume)
                )
            },
            AMQPMethod::ConsumeOk(ref consume_ok) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_consume_ok(consume_ok)
                )
            },
            AMQPMethod::Cancel(ref cancel) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_cancel(cancel)
                )
            },
            AMQPMethod::CancelOk(ref cancel_ok) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_cancel_ok(cancel_ok)
                )
            },
            AMQPMethod::Publish(ref publish) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_publish(publish)
                )
            },
            AMQPMethod::Return(ref return_) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_return_(return_)
                )
            },
            AMQPMethod::Deliver(ref deliver) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_deliver(deliver)
                )
            },
            AMQPMethod::Get(ref get) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_get(get)
                )
            },
            AMQPMethod::GetOk(ref get_ok) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_get_ok(get_ok)
                )
            },
            AMQPMethod::GetEmpty(ref get_empty) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_get_empty(get_empty)
                )
            },
            AMQPMethod::Ack(ref ack) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_ack(ack)
                )
            },
            AMQPMethod::Reject(ref reject) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_reject(reject)
                )
            },
            AMQPMethod::RecoverAsync(ref recover_async) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_recover_async(recover_async)
                )
            },
            AMQPMethod::Recover(ref recover) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_recover(recover)
                )
            },
            AMQPMethod::RecoverOk(ref recover_ok) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_recover_ok(recover_ok)
                )
            },
            AMQPMethod::Nack(ref nack) => {
                do_gen!(input,
                    gen_id(&60) >>
                    gen_nack(nack)
                )
            },
            }
    }

    /// The available methods in basic
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// qos (Generated)
        Qos(Qos),
        /// qos-ok (Generated)
        QosOk(QosOk),
        /// consume (Generated)
        Consume(Consume),
        /// consume-ok (Generated)
        ConsumeOk(ConsumeOk),
        /// cancel (Generated)
        Cancel(Cancel),
        /// cancel-ok (Generated)
        CancelOk(CancelOk),
        /// publish (Generated)
        Publish(Publish),
        /// return (Generated)
        Return(Return),
        /// deliver (Generated)
        Deliver(Deliver),
        /// get (Generated)
        Get(Get),
        /// get-ok (Generated)
        GetOk(GetOk),
        /// get-empty (Generated)
        GetEmpty(GetEmpty),
        /// ack (Generated)
        Ack(Ack),
        /// reject (Generated)
        Reject(Reject),
        /// recover-async (Generated)
        RecoverAsync(RecoverAsync),
        /// recover (Generated)
        Recover(Recover),
        /// recover-ok (Generated)
        RecoverOk(RecoverOk),
        /// nack (Generated)
        Nack(Nack),
        }

    
    /// qos (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Qos {
        /// prefetch-size (Generated)
        pub prefetch_size: LongUInt,
        /// prefetch-count (Generated)
        pub prefetch_count: ShortUInt,
        
        /// global (Generated)
        pub global: Boolean,
        }

    impl Qos {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 10;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse qos (Generated)"], pub parse_qos<Qos>, do_parse!(
        prefetch_size: parse_long_uint >>
        prefetch_count: parse_short_uint >>
        
        flags: apply!(parse_flags, &[
            "global",
            ]) >>
        (Qos {
            prefetch_size,
            prefetch_count,
            
            global: flags.get_flag("global").unwrap_or(false),
            })
    ));

    /// Serialize qos (Generated)
    pub fn gen_qos<'a>(input: (&'a mut [u8], usize), method: &Qos) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("global".to_string(), method.global);
        do_gen!(input,
            gen_id(&10)
            >> gen_long_uint(&method.prefetch_size)
            >> gen_short_uint(&method.prefetch_count)
            
            >> gen_flags(&flags)
            )
    }
    
    /// qos-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct QosOk {
        }

    impl QosOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 11;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse qos-ok (Generated)"], pub parse_qos_ok<QosOk>, do_parse!(
        (QosOk {
            })
    ));

    /// Serialize qos-ok (Generated)
    pub fn gen_qos_ok<'a>(input: (&'a mut [u8], usize), _: &QosOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&11)
            )
    }
    
    /// consume (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Consume {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// queue (Generated)
        pub queue: ShortString,
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
        
        /// no-local (Generated)
        pub no_local: Boolean,
        /// no-ack (Generated)
        pub no_ack: Boolean,
        /// exclusive (Generated)
        pub exclusive: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
        }

    impl Consume {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 20;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse consume (Generated)"], pub parse_consume<Consume>, do_parse!(
        ticket: parse_short_uint >>
        queue: parse_short_string >>
        consumer_tag: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "no-local",
            "no-ack",
            "exclusive",
            "nowait",
            ]) >>
        arguments: parse_field_table >>
        (Consume {
            ticket,
            queue,
            consumer_tag,
            
            no_local: flags.get_flag("no_local").unwrap_or(false),
            no_ack: flags.get_flag("no_ack").unwrap_or(false),
            exclusive: flags.get_flag("exclusive").unwrap_or(false),
            nowait: flags.get_flag("nowait").unwrap_or(false),
            arguments,
            })
    ));

    /// Serialize consume (Generated)
    pub fn gen_consume<'a>(input: (&'a mut [u8], usize), method: &Consume) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("no_local".to_string(), method.no_local);
        flags.add_flag("no_ack".to_string(), method.no_ack);
        flags.add_flag("exclusive".to_string(), method.exclusive);
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&20)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.queue)
            >> gen_short_string(&method.consumer_tag)
            
            >> gen_flags(&flags)
            >> gen_field_table(&method.arguments)
            )
    }
    
    /// consume-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct ConsumeOk {
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
        }

    impl ConsumeOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 21;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse consume-ok (Generated)"], pub parse_consume_ok<ConsumeOk>, do_parse!(
        consumer_tag: parse_short_string >>
        (ConsumeOk {
            consumer_tag,
            })
    ));

    /// Serialize consume-ok (Generated)
    pub fn gen_consume_ok<'a>(input: (&'a mut [u8], usize), method: &ConsumeOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&21)
            >> gen_short_string(&method.consumer_tag)
            )
    }
    
    /// cancel (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Cancel {
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
        
        /// nowait (Generated)
        pub nowait: Boolean,
        }

    impl Cancel {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 30;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse cancel (Generated)"], pub parse_cancel<Cancel>, do_parse!(
        consumer_tag: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "nowait",
            ]) >>
        (Cancel {
            consumer_tag,
            
            nowait: flags.get_flag("nowait").unwrap_or(false),
            })
    ));

    /// Serialize cancel (Generated)
    pub fn gen_cancel<'a>(input: (&'a mut [u8], usize), method: &Cancel) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&30)
            >> gen_short_string(&method.consumer_tag)
            
            >> gen_flags(&flags)
            )
    }
    
    /// cancel-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct CancelOk {
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
        }

    impl CancelOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 31;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse cancel-ok (Generated)"], pub parse_cancel_ok<CancelOk>, do_parse!(
        consumer_tag: parse_short_string >>
        (CancelOk {
            consumer_tag,
            })
    ));

    /// Serialize cancel-ok (Generated)
    pub fn gen_cancel_ok<'a>(input: (&'a mut [u8], usize), method: &CancelOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&31)
            >> gen_short_string(&method.consumer_tag)
            )
    }
    
    /// publish (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Publish {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        
        /// mandatory (Generated)
        pub mandatory: Boolean,
        /// immediate (Generated)
        pub immediate: Boolean,
        }

    impl Publish {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 40;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse publish (Generated)"], pub parse_publish<Publish>, do_parse!(
        ticket: parse_short_uint >>
        exchange: parse_short_string >>
        routing_key: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "mandatory",
            "immediate",
            ]) >>
        (Publish {
            ticket,
            exchange,
            routing_key,
            
            mandatory: flags.get_flag("mandatory").unwrap_or(false),
            immediate: flags.get_flag("immediate").unwrap_or(false),
            })
    ));

    /// Serialize publish (Generated)
    pub fn gen_publish<'a>(input: (&'a mut [u8], usize), method: &Publish) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("mandatory".to_string(), method.mandatory);
        flags.add_flag("immediate".to_string(), method.immediate);
        do_gen!(input,
            gen_id(&40)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.exchange)
            >> gen_short_string(&method.routing_key)
            
            >> gen_flags(&flags)
            )
    }
    
    /// return (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Return {
        /// reply-code (Generated)
        pub reply_code: ShortUInt,
        /// reply-text (Generated)
        pub reply_text: ShortString,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        }

    impl Return {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 50;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse return (Generated)"], pub parse_return_<Return>, do_parse!(
        reply_code: parse_short_uint >>
        reply_text: parse_short_string >>
        exchange: parse_short_string >>
        routing_key: parse_short_string >>
        (Return {
            reply_code,
            reply_text,
            exchange,
            routing_key,
            })
    ));

    /// Serialize return (Generated)
    pub fn gen_return_<'a>(input: (&'a mut [u8], usize), method: &Return) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&50)
            >> gen_short_uint(&method.reply_code)
            >> gen_short_string(&method.reply_text)
            >> gen_short_string(&method.exchange)
            >> gen_short_string(&method.routing_key)
            )
    }
    
    /// deliver (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Deliver {
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        
        /// redelivered (Generated)
        pub redelivered: Boolean,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        }

    impl Deliver {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 60;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse deliver (Generated)"], pub parse_deliver<Deliver>, do_parse!(
        consumer_tag: parse_short_string >>
        delivery_tag: parse_long_long_uint >>
        
        flags: apply!(parse_flags, &[
            "redelivered",
            ]) >>
        exchange: parse_short_string >>
        routing_key: parse_short_string >>
        (Deliver {
            consumer_tag,
            delivery_tag,
            
            redelivered: flags.get_flag("redelivered").unwrap_or(false),
            exchange,
            routing_key,
            })
    ));

    /// Serialize deliver (Generated)
    pub fn gen_deliver<'a>(input: (&'a mut [u8], usize), method: &Deliver) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("redelivered".to_string(), method.redelivered);
        do_gen!(input,
            gen_id(&60)
            >> gen_short_string(&method.consumer_tag)
            >> gen_long_long_uint(&method.delivery_tag)
            
            >> gen_flags(&flags)
            >> gen_short_string(&method.exchange)
            >> gen_short_string(&method.routing_key)
            )
    }
    
    /// get (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Get {
        /// ticket (Generated)
        pub ticket: ShortUInt,
        /// queue (Generated)
        pub queue: ShortString,
        
        /// no-ack (Generated)
        pub no_ack: Boolean,
        }

    impl Get {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 70;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse get (Generated)"], pub parse_get<Get>, do_parse!(
        ticket: parse_short_uint >>
        queue: parse_short_string >>
        
        flags: apply!(parse_flags, &[
            "no-ack",
            ]) >>
        (Get {
            ticket,
            queue,
            
            no_ack: flags.get_flag("no_ack").unwrap_or(false),
            })
    ));

    /// Serialize get (Generated)
    pub fn gen_get<'a>(input: (&'a mut [u8], usize), method: &Get) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("no_ack".to_string(), method.no_ack);
        do_gen!(input,
            gen_id(&70)
            >> gen_short_uint(&method.ticket)
            >> gen_short_string(&method.queue)
            
            >> gen_flags(&flags)
            )
    }
    
    /// get-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct GetOk {
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        
        /// redelivered (Generated)
        pub redelivered: Boolean,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        /// message-count (Generated)
        pub message_count: LongUInt,
        }

    impl GetOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 71;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse get-ok (Generated)"], pub parse_get_ok<GetOk>, do_parse!(
        delivery_tag: parse_long_long_uint >>
        
        flags: apply!(parse_flags, &[
            "redelivered",
            ]) >>
        exchange: parse_short_string >>
        routing_key: parse_short_string >>
        message_count: parse_long_uint >>
        (GetOk {
            delivery_tag,
            
            redelivered: flags.get_flag("redelivered").unwrap_or(false),
            exchange,
            routing_key,
            message_count,
            })
    ));

    /// Serialize get-ok (Generated)
    pub fn gen_get_ok<'a>(input: (&'a mut [u8], usize), method: &GetOk) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("redelivered".to_string(), method.redelivered);
        do_gen!(input,
            gen_id(&71)
            >> gen_long_long_uint(&method.delivery_tag)
            
            >> gen_flags(&flags)
            >> gen_short_string(&method.exchange)
            >> gen_short_string(&method.routing_key)
            >> gen_long_uint(&method.message_count)
            )
    }
    
    /// get-empty (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct GetEmpty {
        /// cluster-id (Generated)
        pub cluster_id: ShortString,
        }

    impl GetEmpty {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 72;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse get-empty (Generated)"], pub parse_get_empty<GetEmpty>, do_parse!(
        cluster_id: parse_short_string >>
        (GetEmpty {
            cluster_id,
            })
    ));

    /// Serialize get-empty (Generated)
    pub fn gen_get_empty<'a>(input: (&'a mut [u8], usize), method: &GetEmpty) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&72)
            >> gen_short_string(&method.cluster_id)
            )
    }
    
    /// ack (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Ack {
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        
        /// multiple (Generated)
        pub multiple: Boolean,
        }

    impl Ack {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 80;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse ack (Generated)"], pub parse_ack<Ack>, do_parse!(
        delivery_tag: parse_long_long_uint >>
        
        flags: apply!(parse_flags, &[
            "multiple",
            ]) >>
        (Ack {
            delivery_tag,
            
            multiple: flags.get_flag("multiple").unwrap_or(false),
            })
    ));

    /// Serialize ack (Generated)
    pub fn gen_ack<'a>(input: (&'a mut [u8], usize), method: &Ack) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("multiple".to_string(), method.multiple);
        do_gen!(input,
            gen_id(&80)
            >> gen_long_long_uint(&method.delivery_tag)
            
            >> gen_flags(&flags)
            )
    }
    
    /// reject (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Reject {
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        
        /// requeue (Generated)
        pub requeue: Boolean,
        }

    impl Reject {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 90;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse reject (Generated)"], pub parse_reject<Reject>, do_parse!(
        delivery_tag: parse_long_long_uint >>
        
        flags: apply!(parse_flags, &[
            "requeue",
            ]) >>
        (Reject {
            delivery_tag,
            
            requeue: flags.get_flag("requeue").unwrap_or(false),
            })
    ));

    /// Serialize reject (Generated)
    pub fn gen_reject<'a>(input: (&'a mut [u8], usize), method: &Reject) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("requeue".to_string(), method.requeue);
        do_gen!(input,
            gen_id(&90)
            >> gen_long_long_uint(&method.delivery_tag)
            
            >> gen_flags(&flags)
            )
    }
    
    /// recover-async (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct RecoverAsync {
        
        /// requeue (Generated)
        pub requeue: Boolean,
        }

    impl RecoverAsync {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 100;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse recover-async (Generated)"], pub parse_recover_async<RecoverAsync>, do_parse!(
        
        flags: apply!(parse_flags, &[
            "requeue",
            ]) >>
        (RecoverAsync {
            
            requeue: flags.get_flag("requeue").unwrap_or(false),
            })
    ));

    /// Serialize recover-async (Generated)
    pub fn gen_recover_async<'a>(input: (&'a mut [u8], usize), method: &RecoverAsync) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("requeue".to_string(), method.requeue);
        do_gen!(input,
            gen_id(&100)
            
            >> gen_flags(&flags)
            )
    }
    
    /// recover (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Recover {
        
        /// requeue (Generated)
        pub requeue: Boolean,
        }

    impl Recover {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 110;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse recover (Generated)"], pub parse_recover<Recover>, do_parse!(
        
        flags: apply!(parse_flags, &[
            "requeue",
            ]) >>
        (Recover {
            
            requeue: flags.get_flag("requeue").unwrap_or(false),
            })
    ));

    /// Serialize recover (Generated)
    pub fn gen_recover<'a>(input: (&'a mut [u8], usize), method: &Recover) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("requeue".to_string(), method.requeue);
        do_gen!(input,
            gen_id(&110)
            
            >> gen_flags(&flags)
            )
    }
    
    /// recover-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct RecoverOk {
        }

    impl RecoverOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 111;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse recover-ok (Generated)"], pub parse_recover_ok<RecoverOk>, do_parse!(
        (RecoverOk {
            })
    ));

    /// Serialize recover-ok (Generated)
    pub fn gen_recover_ok<'a>(input: (&'a mut [u8], usize), _: &RecoverOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&111)
            )
    }
    
    /// nack (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Nack {
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        
        /// multiple (Generated)
        pub multiple: Boolean,
        /// requeue (Generated)
        pub requeue: Boolean,
        }

    impl Nack {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 120;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 60;
        }
    }

    named_attr!(#[doc = "Parse nack (Generated)"], pub parse_nack<Nack>, do_parse!(
        delivery_tag: parse_long_long_uint >>
        
        flags: apply!(parse_flags, &[
            "multiple",
            "requeue",
            ]) >>
        (Nack {
            delivery_tag,
            
            multiple: flags.get_flag("multiple").unwrap_or(false),
            requeue: flags.get_flag("requeue").unwrap_or(false),
            })
    ));

    /// Serialize nack (Generated)
    pub fn gen_nack<'a>(input: (&'a mut [u8], usize), method: &Nack) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("multiple".to_string(), method.multiple);
        flags.add_flag("requeue".to_string(), method.requeue);
        do_gen!(input,
            gen_id(&120)
            >> gen_long_long_uint(&method.delivery_tag)
            
            >> gen_flags(&flags)
            )
    }
    /// basic properties (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct AMQPProperties {
        content_type: Option<ShortString>,
        content_encoding: Option<ShortString>,
        headers: Option<FieldTable>,
        delivery_mode: Option<ShortShortUInt>,
        priority: Option<ShortShortUInt>,
        correlation_id: Option<ShortString>,
        reply_to: Option<ShortString>,
        expiration: Option<ShortString>,
        message_id: Option<ShortString>,
        timestamp: Option<Timestamp>,
        type_: Option<ShortString>,
        user_id: Option<ShortString>,
        app_id: Option<ShortString>,
        cluster_id: Option<ShortString>,
        }

    impl Default for AMQPProperties {
        fn default() -> AMQPProperties {
            AMQPProperties {
                content_type: None,
                content_encoding: None,
                headers: None,
                delivery_mode: None,
                priority: None,
                correlation_id: None,
                reply_to: None,
                expiration: None,
                message_id: None,
                timestamp: None,
                type_: None,
                user_id: None,
                app_id: None,
                cluster_id: None,
                }
        }
    }

    impl AMQPProperties {
        /// Set content-type (Generated)
        pub fn with_content_type(mut self, value: ShortString) -> AMQPProperties {
            self.content_type = Some(value);
            self
        }
        /// Set content-encoding (Generated)
        pub fn with_content_encoding(mut self, value: ShortString) -> AMQPProperties {
            self.content_encoding = Some(value);
            self
        }
        /// Set headers (Generated)
        pub fn with_headers(mut self, value: FieldTable) -> AMQPProperties {
            self.headers = Some(value);
            self
        }
        /// Set delivery-mode (Generated)
        pub fn with_delivery_mode(mut self, value: ShortShortUInt) -> AMQPProperties {
            self.delivery_mode = Some(value);
            self
        }
        /// Set priority (Generated)
        pub fn with_priority(mut self, value: ShortShortUInt) -> AMQPProperties {
            self.priority = Some(value);
            self
        }
        /// Set correlation-id (Generated)
        pub fn with_correlation_id(mut self, value: ShortString) -> AMQPProperties {
            self.correlation_id = Some(value);
            self
        }
        /// Set reply-to (Generated)
        pub fn with_reply_to(mut self, value: ShortString) -> AMQPProperties {
            self.reply_to = Some(value);
            self
        }
        /// Set expiration (Generated)
        pub fn with_expiration(mut self, value: ShortString) -> AMQPProperties {
            self.expiration = Some(value);
            self
        }
        /// Set message-id (Generated)
        pub fn with_message_id(mut self, value: ShortString) -> AMQPProperties {
            self.message_id = Some(value);
            self
        }
        /// Set timestamp (Generated)
        pub fn with_timestamp(mut self, value: Timestamp) -> AMQPProperties {
            self.timestamp = Some(value);
            self
        }
        /// Set type (Generated)
        pub fn with_type_(mut self, value: ShortString) -> AMQPProperties {
            self.type_ = Some(value);
            self
        }
        /// Set user-id (Generated)
        pub fn with_user_id(mut self, value: ShortString) -> AMQPProperties {
            self.user_id = Some(value);
            self
        }
        /// Set app-id (Generated)
        pub fn with_app_id(mut self, value: ShortString) -> AMQPProperties {
            self.app_id = Some(value);
            self
        }
        /// Set cluster-id (Generated)
        pub fn with_cluster_id(mut self, value: ShortString) -> AMQPProperties {
            self.cluster_id = Some(value);
            self
        }
        /// Get content-type (Generated)
        pub fn content_type(&self) -> &Option<ShortString> {
            &self.content_type
        }
        /// Get content-encoding (Generated)
        pub fn content_encoding(&self) -> &Option<ShortString> {
            &self.content_encoding
        }
        /// Get headers (Generated)
        pub fn headers(&self) -> &Option<FieldTable> {
            &self.headers
        }
        /// Get delivery-mode (Generated)
        pub fn delivery_mode(&self) -> &Option<ShortShortUInt> {
            &self.delivery_mode
        }
        /// Get priority (Generated)
        pub fn priority(&self) -> &Option<ShortShortUInt> {
            &self.priority
        }
        /// Get correlation-id (Generated)
        pub fn correlation_id(&self) -> &Option<ShortString> {
            &self.correlation_id
        }
        /// Get reply-to (Generated)
        pub fn reply_to(&self) -> &Option<ShortString> {
            &self.reply_to
        }
        /// Get expiration (Generated)
        pub fn expiration(&self) -> &Option<ShortString> {
            &self.expiration
        }
        /// Get message-id (Generated)
        pub fn message_id(&self) -> &Option<ShortString> {
            &self.message_id
        }
        /// Get timestamp (Generated)
        pub fn timestamp(&self) -> &Option<Timestamp> {
            &self.timestamp
        }
        /// Get type (Generated)
        pub fn type_(&self) -> &Option<ShortString> {
            &self.type_
        }
        /// Get user-id (Generated)
        pub fn user_id(&self) -> &Option<ShortString> {
            &self.user_id
        }
        /// Get app-id (Generated)
        pub fn app_id(&self) -> &Option<ShortString> {
            &self.app_id
        }
        /// Get cluster-id (Generated)
        pub fn cluster_id(&self) -> &Option<ShortString> {
            &self.cluster_id
        }
        /// Get the bitpask for serialization (Generated)
        pub fn bitmask(&self) -> ShortUInt {
            (if self.content_type.is_some() { 1 << (15 - 0) } else { 0 }) + (if self.content_encoding.is_some() { 1 << (15 - 1) } else { 0 }) + (if self.headers.is_some() { 1 << (15 - 2) } else { 0 }) + (if self.delivery_mode.is_some() { 1 << (15 - 3) } else { 0 }) + (if self.priority.is_some() { 1 << (15 - 4) } else { 0 }) + (if self.correlation_id.is_some() { 1 << (15 - 5) } else { 0 }) + (if self.reply_to.is_some() { 1 << (15 - 6) } else { 0 }) + (if self.expiration.is_some() { 1 << (15 - 7) } else { 0 }) + (if self.message_id.is_some() { 1 << (15 - 8) } else { 0 }) + (if self.timestamp.is_some() { 1 << (15 - 9) } else { 0 }) + (if self.type_.is_some() { 1 << (15 - 10) } else { 0 }) + (if self.user_id.is_some() { 1 << (15 - 11) } else { 0 }) + (if self.app_id.is_some() { 1 << (15 - 12) } else { 0 }) + (if self.cluster_id.is_some() { 1 << (15 - 13) } else { 0 }) }
    }

    named_attr!(#[doc = "Parse basic properties (Generated)"], pub parse_properties<AMQPProperties>, do_parse!(
        flags: parse_short_uint >>
        content_type: cond!(flags & (1 << (15 - 0)) != 0, parse_short_string) >>
        content_encoding: cond!(flags & (1 << (15 - 1)) != 0, parse_short_string) >>
        headers: cond!(flags & (1 << (15 - 2)) != 0, parse_field_table) >>
        delivery_mode: cond!(flags & (1 << (15 - 3)) != 0, parse_short_short_uint) >>
        priority: cond!(flags & (1 << (15 - 4)) != 0, parse_short_short_uint) >>
        correlation_id: cond!(flags & (1 << (15 - 5)) != 0, parse_short_string) >>
        reply_to: cond!(flags & (1 << (15 - 6)) != 0, parse_short_string) >>
        expiration: cond!(flags & (1 << (15 - 7)) != 0, parse_short_string) >>
        message_id: cond!(flags & (1 << (15 - 8)) != 0, parse_short_string) >>
        timestamp: cond!(flags & (1 << (15 - 9)) != 0, parse_timestamp) >>
        type_: cond!(flags & (1 << (15 - 10)) != 0, parse_short_string) >>
        user_id: cond!(flags & (1 << (15 - 11)) != 0, parse_short_string) >>
        app_id: cond!(flags & (1 << (15 - 12)) != 0, parse_short_string) >>
        cluster_id: cond!(flags & (1 << (15 - 13)) != 0, parse_short_string) >>
        (AMQPProperties {
            content_type,
            content_encoding,
            headers,
            delivery_mode,
            priority,
            correlation_id,
            reply_to,
            expiration,
            message_id,
            timestamp,
            type_,
            user_id,
            app_id,
            cluster_id,
            })
    ));

    /// Serialize basic properties (Generated)
    pub fn gen_properties<'a>(input:(&'a mut [u8],usize), props: &AMQPProperties) -> Result<(&'a mut [u8],usize),GenError> {
        do_gen!(input,
            gen_short_uint(&props.bitmask())
            >> gen_cond!(props.content_type.is_some(), gen_call!(gen_short_string, &props.content_type.as_ref().unwrap()))
            >> gen_cond!(props.content_encoding.is_some(), gen_call!(gen_short_string, &props.content_encoding.as_ref().unwrap()))
            >> gen_cond!(props.headers.is_some(), gen_call!(gen_field_table, &props.headers.as_ref().unwrap()))
            >> gen_cond!(props.delivery_mode.is_some(), gen_call!(gen_short_short_uint, &props.delivery_mode.as_ref().unwrap()))
            >> gen_cond!(props.priority.is_some(), gen_call!(gen_short_short_uint, &props.priority.as_ref().unwrap()))
            >> gen_cond!(props.correlation_id.is_some(), gen_call!(gen_short_string, &props.correlation_id.as_ref().unwrap()))
            >> gen_cond!(props.reply_to.is_some(), gen_call!(gen_short_string, &props.reply_to.as_ref().unwrap()))
            >> gen_cond!(props.expiration.is_some(), gen_call!(gen_short_string, &props.expiration.as_ref().unwrap()))
            >> gen_cond!(props.message_id.is_some(), gen_call!(gen_short_string, &props.message_id.as_ref().unwrap()))
            >> gen_cond!(props.timestamp.is_some(), gen_call!(gen_timestamp, &props.timestamp.as_ref().unwrap()))
            >> gen_cond!(props.type_.is_some(), gen_call!(gen_short_string, &props.type_.as_ref().unwrap()))
            >> gen_cond!(props.user_id.is_some(), gen_call!(gen_short_string, &props.user_id.as_ref().unwrap()))
            >> gen_cond!(props.app_id.is_some(), gen_call!(gen_short_string, &props.app_id.as_ref().unwrap()))
            >> gen_cond!(props.cluster_id.is_some(), gen_call!(gen_short_string, &props.cluster_id.as_ref().unwrap()))
            )
    }
    }

/// tx (generated)
pub mod tx {
    use super::*;

    /// Get the id of the AMQP class
    pub fn get_id() -> ShortUInt {
        return 90;
    }

    named_attr!(#[doc = "Parse tx (Generated)"], pub parse_tx<tx::AMQPMethod>, switch!(parse_id,
        10 => map!(call!(parse_select), AMQPMethod::Select) |11 => map!(call!(parse_select_ok), AMQPMethod::SelectOk) |20 => map!(call!(parse_commit), AMQPMethod::Commit) |21 => map!(call!(parse_commit_ok), AMQPMethod::CommitOk) |30 => map!(call!(parse_rollback), AMQPMethod::Rollback) |31 => map!(call!(parse_rollback_ok), AMQPMethod::RollbackOk) ));

    /// Serialize tx (Generated)
    pub fn gen_tx<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            AMQPMethod::Select(ref select) => {
                do_gen!(input,
                    gen_id(&90) >>
                    gen_select(select)
                )
            },
            AMQPMethod::SelectOk(ref select_ok) => {
                do_gen!(input,
                    gen_id(&90) >>
                    gen_select_ok(select_ok)
                )
            },
            AMQPMethod::Commit(ref commit) => {
                do_gen!(input,
                    gen_id(&90) >>
                    gen_commit(commit)
                )
            },
            AMQPMethod::CommitOk(ref commit_ok) => {
                do_gen!(input,
                    gen_id(&90) >>
                    gen_commit_ok(commit_ok)
                )
            },
            AMQPMethod::Rollback(ref rollback) => {
                do_gen!(input,
                    gen_id(&90) >>
                    gen_rollback(rollback)
                )
            },
            AMQPMethod::RollbackOk(ref rollback_ok) => {
                do_gen!(input,
                    gen_id(&90) >>
                    gen_rollback_ok(rollback_ok)
                )
            },
            }
    }

    /// The available methods in tx
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// select (Generated)
        Select(Select),
        /// select-ok (Generated)
        SelectOk(SelectOk),
        /// commit (Generated)
        Commit(Commit),
        /// commit-ok (Generated)
        CommitOk(CommitOk),
        /// rollback (Generated)
        Rollback(Rollback),
        /// rollback-ok (Generated)
        RollbackOk(RollbackOk),
        }

    
    /// select (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Select {
        }

    impl Select {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 10;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 90;
        }
    }

    named_attr!(#[doc = "Parse select (Generated)"], pub parse_select<Select>, do_parse!(
        (Select {
            })
    ));

    /// Serialize select (Generated)
    pub fn gen_select<'a>(input: (&'a mut [u8], usize), _: &Select) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&10)
            )
    }
    
    /// select-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct SelectOk {
        }

    impl SelectOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 11;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 90;
        }
    }

    named_attr!(#[doc = "Parse select-ok (Generated)"], pub parse_select_ok<SelectOk>, do_parse!(
        (SelectOk {
            })
    ));

    /// Serialize select-ok (Generated)
    pub fn gen_select_ok<'a>(input: (&'a mut [u8], usize), _: &SelectOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&11)
            )
    }
    
    /// commit (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Commit {
        }

    impl Commit {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 20;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 90;
        }
    }

    named_attr!(#[doc = "Parse commit (Generated)"], pub parse_commit<Commit>, do_parse!(
        (Commit {
            })
    ));

    /// Serialize commit (Generated)
    pub fn gen_commit<'a>(input: (&'a mut [u8], usize), _: &Commit) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&20)
            )
    }
    
    /// commit-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct CommitOk {
        }

    impl CommitOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 21;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 90;
        }
    }

    named_attr!(#[doc = "Parse commit-ok (Generated)"], pub parse_commit_ok<CommitOk>, do_parse!(
        (CommitOk {
            })
    ));

    /// Serialize commit-ok (Generated)
    pub fn gen_commit_ok<'a>(input: (&'a mut [u8], usize), _: &CommitOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&21)
            )
    }
    
    /// rollback (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Rollback {
        }

    impl Rollback {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 30;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 90;
        }
    }

    named_attr!(#[doc = "Parse rollback (Generated)"], pub parse_rollback<Rollback>, do_parse!(
        (Rollback {
            })
    ));

    /// Serialize rollback (Generated)
    pub fn gen_rollback<'a>(input: (&'a mut [u8], usize), _: &Rollback) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&30)
            )
    }
    
    /// rollback-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct RollbackOk {
        }

    impl RollbackOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 31;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 90;
        }
    }

    named_attr!(#[doc = "Parse rollback-ok (Generated)"], pub parse_rollback_ok<RollbackOk>, do_parse!(
        (RollbackOk {
            })
    ));

    /// Serialize rollback-ok (Generated)
    pub fn gen_rollback_ok<'a>(input: (&'a mut [u8], usize), _: &RollbackOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&31)
            )
    }
    }

/// confirm (generated)
pub mod confirm {
    use super::*;

    /// Get the id of the AMQP class
    pub fn get_id() -> ShortUInt {
        return 85;
    }

    named_attr!(#[doc = "Parse confirm (Generated)"], pub parse_confirm<confirm::AMQPMethod>, switch!(parse_id,
        10 => map!(call!(parse_select), AMQPMethod::Select) |11 => map!(call!(parse_select_ok), AMQPMethod::SelectOk) ));

    /// Serialize confirm (Generated)
    pub fn gen_confirm<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            AMQPMethod::Select(ref select) => {
                do_gen!(input,
                    gen_id(&85) >>
                    gen_select(select)
                )
            },
            AMQPMethod::SelectOk(ref select_ok) => {
                do_gen!(input,
                    gen_id(&85) >>
                    gen_select_ok(select_ok)
                )
            },
            }
    }

    /// The available methods in confirm
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// select (Generated)
        Select(Select),
        /// select-ok (Generated)
        SelectOk(SelectOk),
        }

    
    /// select (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct Select {
        
        /// nowait (Generated)
        pub nowait: Boolean,
        }

    impl Select {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 10;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 85;
        }
    }

    named_attr!(#[doc = "Parse select (Generated)"], pub parse_select<Select>, do_parse!(
        
        flags: apply!(parse_flags, &[
            "nowait",
            ]) >>
        (Select {
            
            nowait: flags.get_flag("nowait").unwrap_or(false),
            })
    ));

    /// Serialize select (Generated)
    pub fn gen_select<'a>(input: (&'a mut [u8], usize), method: &Select) -> Result<(&'a mut [u8],usize), GenError> {
        let mut flags = AMQPFlags::default();
        flags.add_flag("nowait".to_string(), method.nowait);
        do_gen!(input,
            gen_id(&10)
            
            >> gen_flags(&flags)
            )
    }
    
    /// select-ok (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct SelectOk {
        }

    impl SelectOk {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return 11;
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return 85;
        }
    }

    named_attr!(#[doc = "Parse select-ok (Generated)"], pub parse_select_ok<SelectOk>, do_parse!(
        (SelectOk {
            })
    ));

    /// Serialize select-ok (Generated)
    pub fn gen_select_ok<'a>(input: (&'a mut [u8], usize), _: &SelectOk) -> Result<(&'a mut [u8],usize), GenError> {
        do_gen!(input,
            gen_id(&11)
            )
    }
    }

