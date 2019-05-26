# jstemplate2
a jobsystem template is  an ayclic graph whose nodes represent potential directories within the file system.

```rust
enum Valid {
   Simple(String),
   Expr(Regex)
}

EntryType {
   Directory,
   Volume,
}

type Id = u32;
struct Node {
  id: Id
  name: ValidName,
  children: Vec<Id>
  type: EntryType
}

impl Node {

}
```
