#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum MessageBasicType
{
  Debug,
  Info,
  Warn,
  Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct MessageType
{
  basic_type: MessageBasicType,
  name:       String,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct MessageContent
{
  title:    String,
  content:  String,
  metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Metadata {}

// # Messages
//
// ## Message Type
//
// Basic type, does not support sending:
//
// + `Debug`
// + `Info`
// + `Warn`
// + `Error`
//
// To define message types, you must choose to inherit from a message type
//
// ## Message Content
//
// Needs to implement [Message Trait](messages/message_trait.md)
//
// ## Message Metadata
//
// + Status (`status`):
// + `Succeed`
// + `Failed`
// + Process (`process`): 0% ~ 100%
// + Progress detail (`progress_detail`)：`0/32`, `loading...`
//
// ## Message Sender
//
// + [Message Type](#message-type)
// + [Message Content](#message-content)
// + Sending time
// + Private ID of the message sending module (`sending_module_key`):
// Needs [Module Private ID](./modules.md#module-private-id)
// + [Message Metadata](#message-metadata)
//
// ## Message
//
// + [Message Type](#message-type)
// + [Message Content](#message-content)
// + Sending time
// + ID of the message sending module (`sending_module_id`): [Module
//   ID](./modules.md#module-id)
// + [Message Metadata](#message-metadata)
//
// ## Message interface
//
// + Send message function
// + Register message type functions
// + Register the message hook function

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn message_basic_type_derives()
  {
    // Copy / Clone / Eq / Ord / Hash are all derived; verify basic value semantics here
    let a = MessageBasicType::Warn;
    let b = a; // Copy
    assert_eq!(a, b);
    assert!(MessageBasicType::Debug < MessageBasicType::Info);
    assert!(MessageBasicType::Info < MessageBasicType::Warn);
    assert!(MessageBasicType::Warn < MessageBasicType::Error);
  }

  #[test]
  fn message_type_construction_and_order()
  {
    let m1 = MessageType { basic_type: MessageBasicType::Info, name: String::from("a") };
    let m2 = MessageType { basic_type: MessageBasicType::Info, name: String::from("b") };
    // Name ordering: a < b
    assert!(m1 < m2);
    // Same name but different basic type: basic_type is compared first
    let m3 = MessageType { basic_type: MessageBasicType::Error, name: String::from("a") };
    assert!(m1 < m3);
  }

  #[test]
  fn message_type_clone_and_eq()
  {
    let m1 = MessageType { basic_type: MessageBasicType::Debug, name: String::from("x") };
    let m2 = m1.clone();
    assert_eq!(m1, m2);
    assert_eq!(hash(&m1), hash(&m2));
  }

  fn hash<T: std::hash::Hash>(v: &T) -> u64
  {
    use std::hash::{DefaultHasher, Hasher};
    let mut h = DefaultHasher::new();
    v.hash(&mut h);
    h.finish()
  }

  #[test]
  fn message_content_fields()
  {
    let content = MessageContent {
      title:   String::from("title"),
      content: String::from("body"),
      metadata: Metadata {},
    };
    assert_eq!(content.title, "title");
    assert_eq!(content.content, "body");
    // Equal after clone
    assert_eq!(content.clone(), content);
  }
}