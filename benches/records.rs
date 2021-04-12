use criterion::{criterion_group, criterion_main, Criterion};

use utilities::{bench_args, EvalMode};

fn count_letters(c: &mut Criterion) {
    bench_args("countLetters", env!("CARGO_MANIFEST_DIR"), "records/countLetters", None,
        vec![String::from(r#"
            "
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Sed vulputate mi sit amet. Et odio pellentesque diam volutpat. Pharetra et ultrices neque ornare. Aenean pharetra magna ac placerat vestibulum lectus mauris ultrices. Iaculis nunc sed augue lacus viverra vitae congue eu. Semper eget duis at tellus at. Elementum sagittis vitae et leo duis ut diam quam nulla. Nisl pretium fusce id velit ut. Magnis dis parturient montes nascetur. Leo duis ut diam quam nulla porttitor massa. Non diam phasellus vestibulum lorem sed risus. Adipiscing vitae proin sagittis nisl rhoncus mattis. Imperdiet dui accumsan sit amet nulla. Venenatis a condimentum vitae sapien pellentesque habitant morbi tristique senectus.

        Sollicitudin ac orci phasellus egestas tellus rutrum tellus pellentesque. Venenatis cras sed felis eget velit. Tortor pretium viverra suspendisse potenti nullam ac. Pretium vulputate sapien nec sagittis aliquam malesuada. Felis eget velit aliquet sagittis id consectetur. Non consectetur a erat nam at lectus urna. Habitant morbi tristique senectus et netus. Praesent semper feugiat nibh sed. Maecenas accumsan lacus vel facilisis volutpat est velit egestas dui. Facilisi etiam dignissim diam quis. Vitae congue eu consequat ac felis donec. Maecenas sed enim ut sem viverra aliquet eget sit. Pulvinar etiam non quam lacus suspendisse faucibus. Donec pretium vulputate sapien nec sagittis aliquam malesuada. Aliquam sem fringilla ut morbi tincidunt augue interdum. Urna duis convallis convallis tellus id interdum velit laoreet.

        Mauris vitae ultricies leo integer malesuada. Sit amet commodo nulla facilisi nullam vehicula ipsum. Amet nulla facilisi morbi tempus iaculis urna id. Ipsum dolor sit amet consectetur. Facilisi morbi tempus iaculis urna id volutpat lacus. Eleifend quam adipiscing vitae proin sagittis nisl rhoncus mattis. Massa sapien faucibus et molestie ac. Erat nam at lectus urna duis convallis convallis. Viverra nam libero justo laoreet sit amet cursus sit amet. Et egestas quis ipsum suspendisse ultrices gravida dictum fusce ut. Molestie ac feugiat sed lectus vestibulum mattis ullamcorper. Odio facilisis mauris sit amet massa vitae tortor condimentum. Mi in nulla posuere sollicitudin aliquam ultrices sagittis orci. Semper risus in hendrerit gravida rutrum quisque non. Nam at lectus urna duis. Aliquet sagittis id consectetur purus ut faucibus pulvinar. Massa massa ultricies mi quis hendrerit dolor magna. Blandit cursus risus at ultrices mi tempus imperdiet nulla malesuada. Quam id leo in vitae turpis massa sed.

        Enim diam vulputate ut pharetra sit amet. Praesent semper feugiat nibh sed pulvinar proin gravida hendrerit. Non blandit massa enim nec. Fringilla ut morbi tincidunt augue interdum velit. Sodales ut eu sem integer vitae justo eget magna fermentum. Sed ullamcorper morbi tincidunt ornare massa eget egestas purus viverra. Sed augue lacus viverra vitae congue. Nunc sed id semper risus in. Sagittis nisl rhoncus mattis rhoncus urna neque viverra. Ullamcorper a lacus vestibulum sed arcu non odio euismod lacinia. Condimentum lacinia quis vel eros donec ac. Non enim praesent elementum facilisis leo vel fringilla est. Facilisi cras fermentum odio eu feugiat pretium nibh. Bibendum ut tristique et egestas quis ipsum suspendisse ultrices. Tristique et egestas quis ipsum suspendisse ultrices gravida. Non curabitur gravida arcu ac tortor dignissim.

        Commodo sed egestas egestas fringilla. Consectetur adipiscing elit pellentesque habitant morbi tristique. Morbi blandit cursus risus at ultrices mi tempus. Odio tempor orci dapibus ultrices in iaculis. Fames ac turpis egestas sed tempus urna et. Nullam vehicula ipsum a arcu cursus vitae. Elementum curabitur vitae nunc sed velit dignissim sodales ut eu. Suscipit tellus mauris a diam maecenas sed. Convallis aenean et tortor at. Varius vel pharetra vel turpis nunc. In tellus integer feugiat scelerisque varius morbi enim nunc. Facilisis magna etiam tempor orci eu. Ut etiam sit amet nisl purus in mollis nunc. Enim neque volutpat ac tincidunt vitae semper quis lectus nulla. Bibendum neque egestas congue quisque.
                "
        "#)],
        EvalMode::DeepSeq, c);
}

fn merge(c: &mut Criterion) {
    bench_args("merge", env!("CARGO_MANIFEST_DIR"), "records/merge", None, vec![String::from("500"), String::from("50")], EvalMode::DeepSeq, c);
}

criterion_group!(benches, count_letters, merge);
criterion_main!(benches);
