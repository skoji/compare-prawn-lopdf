#[macro_use]
extern crate lopdf;
use lopdf::{Document, Object, Stream};
use lopdf::content::{Content, Operation};
fn hello() {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Courier",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 48.into()]),
            Operation::new("Td", vec![100.into(), 600.into()]),
            Operation::new("Tj", vec![Object::string_literal("Hello World!")]),
            Operation::new("ET", vec![]),
        ],
    };
    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });
    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.save("hello-lopdf.pdf").unwrap();
}
fn outline() {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Courier",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });

    let mut pages_list: Vec<Object> = Vec::new();
    
    for x in 0..3 {
        let str = format!("Page {}", x + 1);
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 48.into()]),
                Operation::new("Td", vec![100.into(), 600.into()]),
                Operation::new("Tj", vec![Object::string_literal(str)]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        });
        pages_list.push(page_id.into());
    }
    let outline_id = {
        let outline_id = doc.new_object_id();
        let outline_first_id = doc.new_object_id();
        let outline_second_id = doc.new_object_id();
        let outline_third_id = doc.new_object_id();

        
        let action_first_id = doc.add_object(dictionary!{
            "D" => vec![pages_list[0].clone(), "FitH".into(), Object::Null],
            "S" => "GoTo"
        });
        let action_second_id = doc.add_object(dictionary!{
            "D" => vec![pages_list[1].clone(), "FitH".into(), Object::Null],
            "S" => "GoTo"
        });
        let action_third_id = doc.add_object(dictionary!{
            "D" => vec![pages_list[2].clone(), "FitH".into(), Object::Null],
            "S" => "GoTo"
        });
        
        let outline_second = dictionary! {
            "Title" => Object::string_literal("Page 2"),
            "Parent" => outline_first_id,
            "A" => action_second_id,
            "Next" => outline_third_id
        };
        let outline_third = dictionary! {
            "Title" => Object::string_literal("Page 3"),
            "Parent" => outline_first_id,
            "A" => action_third_id,
            "Prev" => outline_second_id
        };
        let outline_first = dictionary! {
            "Title" => Object::string_literal("Section 1"),
            "Parent" => outline_id,
            "A" => action_first_id,
            "First" => outline_second_id,
            "Last" => outline_third_id
        };
        let outline = dictionary! {
            "Count" => 2,
            "First" => outline_first_id,
            "Last" => outline_first_id,
        };
        doc.objects.insert(outline_first_id, Object::Dictionary(outline_first));
        doc.objects.insert(outline_second_id, Object::Dictionary(outline_second));
        doc.objects.insert(outline_third_id, Object::Dictionary(outline_third));
        doc.objects.insert(outline_id, Object::Dictionary(outline));
        outline_id
    };
    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => pages_list,
        "Count" => 3, 
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
        "Outlines" => outline_id
    });
    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.save("outline.pdf").unwrap();
}

fn main() {
    hello();
    outline();
}
