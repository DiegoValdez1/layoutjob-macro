# LayoutJob Macro

This is a rust crate which has a single macro: `layout!{...}`. This allows for an easy construction of a `LayoutJob` from epaint (which is re-exported in egui).

## Usage

```rust
layout!{
    default_format;
    text leading_space <text_format>,
    text leading_space <text_format>,
    ...
}
```

Where:
- `default_format` must be identifier to a epaint `TextFormat`. Is optional. If present then semicolon is required after.
- `text` can be a literal, ident, or borrowed ident which must lead to a `&str`. Is required.
- `leading_space` can be an literal, ident, or borrowed ident which must lead to a `f32`. Optional.
- `text_format` must be an identifier to a epaint `TextFormat`. Optional.

## Notes
- If `default_format` is not present, then `Default::default()` is used.
- If for a certain line, a format is not present, then `default_format` is used.
- All the `TextFormats` are cloned.

## Examples
```rust
let fmt = TextFormat {
    color: Color32::from_rgb(255, 0, 0),
    ..Default::default()
}
let job: LayoutJob = layout!{
    "Hello",
    "World" 1.0 <fmt>
};
```

```rust
let string = "This is a string!".to_string();
let str_slice = "This is a slice of a string!";

let space = 2.0;

let default_fmt = TextFormat {
    color: Color32::RED,
    ..Default::default()
};
let secondary_fmt = TextFormat {
    color: Color32::GREEN,
    italics: true,
    ..Default::default()
};

let manual_job: LayoutJob = {
    let mut job = LayoutJob::default();
    job.append("Hello", 0.0, default_fmt.clone());
    job.append("World!", 1.0, secondary_fmt.clone());
    job.append(&string, space, default_fmt.clone());
    job.append(str_slice, space, secondary_fmt.clone());
    job
};

let macro_job: LayoutJob = layout!(
    default_fmt;
    "Hello",
    "World!" 1.0 <secondary_fmt>,
    &string space,
    str_slice space <secondary_fmt>
);

assert_eq!(manual_job, macro_job) // Passes
```