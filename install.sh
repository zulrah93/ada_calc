    git clone https://github.com/zulrah93/ada_calc.git
    cd ada_calc
    cargo build --release
    echo "Installing ada_calc in sbin!"
    cp /target/release/ada_calc /sbin/ada_calc
    echo "Restart terminal console for the changes to take effect!"
    