mod source;

use source::kafka_source;

fn main() {
    let a  = kafka_source {
        host: "123"
    };

    a.set();
}
