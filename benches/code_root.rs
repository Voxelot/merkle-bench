use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fuel_merkle::binary::in_memory::MerkleTree;
use fuel_types::Bytes32;

criterion_group!(benches, code_roots);
criterion_main!(benches);

fn code_roots(c: &mut Criterion) {
    // 16 KiB of code to merklize
    let bytes = vec![0u8; 16 * 1024 * 1024];
    c.bench_function("word sized chunks", |b| {
        b.iter(|| merkle_adjustable_chunk_size::<8>(black_box(&bytes)))
    });
    c.bench_function("quad word sized chunks", |b| {
        b.iter(|| merkle_adjustable_chunk_size::<32>(black_box(&bytes)))
    });
    c.bench_function("1 kib sized chunks", |b| {
        b.iter(|| merkle_adjustable_chunk_size::<1024>(black_box(&bytes)))
    });
    c.bench_function("page sized chunks", |b| {
        b.iter(|| merkle_adjustable_chunk_size::<4096>(black_box(&bytes)))
    });
}

fn merkle_adjustable_chunk_size<const CHUNK_SIZE: usize>(bytes: &[u8]) -> Bytes32 {
    let mut tree = MerkleTree::new();

    bytes
        .as_ref()
        .chunks(CHUNK_SIZE)
        .map(|c| {
            if c.len() == CHUNK_SIZE {
                // Safety: checked len chunk
                <[u8; CHUNK_SIZE]>::try_from(c).unwrap()
            } else {
                // Potential collision with non-padded input. Consider adding an extra leaf
                // for padding?
                let mut b = [0u8; CHUNK_SIZE];

                let l = c.len();
                b[..l].copy_from_slice(c);

                b.into()
            }
        })
        .for_each(|l| tree.push(l.as_ref()));

    tree.root().into()
}
