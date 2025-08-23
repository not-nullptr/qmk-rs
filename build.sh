sudo mkdir /mnt/h
sudo mount -t drvfs I: /mnt/h
rm -rf .build & rm rust/rust_keymap.a & SILENT= CONVERT_TO=elite_pi make VERBOSE=true ALLOW_WARNINGS=yes CONSOLE_ENABLE=yes sofle/rev1:nulls_keymap && cp sofle_rev1_nulls_keymap_elite_pi.uf2 /mnt/h/
