# llhttp-rs

llhttp bindings for Rust

## Quick starts

```rust
use llhttp_rs::*;

fn main() {
  #[derive(Default)]
  struct CallbackList {
      called_on_url: bool,
      called_on_header_field: bool,
      called_on_header_value: bool,
      called_on_body: bool,
  }

  impl Callbacks for CallbackList {
      fn on_url(&mut self, _: &mut Parser, url: &[u8]) -> ParserResult<()> {
          assert_eq!(b"/say_hello", url);
          self.called_on_url = true;
          Ok(())
      }

      fn on_header_field(&mut self, _: &mut Parser, hdr: &[u8]) -> ParserResult<()> {
          assert!(hdr == b"Host" || hdr == b"Content-Length");
          self.called_on_header_field = true;
          Ok(())
      }

      fn on_header_value(&mut self, _: &mut Parser, val: &[u8]) -> ParserResult<()> {
          assert!(val == b"localhost.localdomain" || val == b"11");
          self.called_on_header_value = true;
          Ok(())
      }

      fn on_body(&mut self, parser: &mut Parser, body: &[u8]) -> ParserResult<()> {
          assert_eq!(parser.get_method(), Some(http::method::Method::POST));
          assert_eq!(parser.get_version(), Some(http::version::Version::HTTP_11));
          assert_eq!(parser.get_status_code(), None);
          assert_eq!(parser.get_upgrade(), false);
          assert_eq!(body, b"Hello world");
          self.called_on_body = true;
          Ok(())
      }
  }

  let req = b"POST /say_hello HTTP/1.1\r\nContent-Length: 11\r\nHost: localhost.localdomain\r\n\r\nHello world";
  let mut handler = CallbackList::default();
  let mut parser = Parser::request();

  parser.parse(&mut handler, req).unwrap();
}
```
