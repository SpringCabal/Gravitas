Fork implementing a more direct control scheme:
- Move with arrows keys
- Push with Left Mouse Button
- Pull with Right Mouse Button

World-space SimpleParticles2 and JitterParticles2 share persistent GPU instance
streams. Particle birth times keep curve evaluation on the GPU; new births,
visibility changes and expired spans update the stream. Ring compaction reuses
capacity. Unit, piece and projectile-relative effects retain their original
transform path. Lups flushes the shared streams before changing transforms.
