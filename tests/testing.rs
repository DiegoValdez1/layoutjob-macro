use epaint::{
    text::{LayoutJob, TextFormat},
    Color32,
};
use layoutjob_macro::layout;

#[test]
fn test_macro_usage() {
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

    assert_eq!(manual_job, macro_job)
}
