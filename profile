open $(cargo instruments --release text -n 1 2>&1 | tail -n 1 | cut -d\  -f4)
