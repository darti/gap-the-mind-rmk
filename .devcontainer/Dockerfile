FROM rust:1.67


RUN \
    apt-get update && \
    apt-get install -y build-essential gcc-arm-linux-gnueabihf zsh locales

RUN locale-gen en_US.UTF-8  

ENV LANG en_US.UTF-8  
ENV LANGUAGE en_US:en  
ENV LC_ALL en_US.UTF-8  

RUN \
    chsh -s $(which zsh) && \
    sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended 
# sed -i 's/ZSH_THEME=.*/ZSH_THEME=\"agnoster\"/' ~/.zshrc

RUN \
    rustup target add armv7-unknown-linux-gnueabihf && \
    rustup component add rustfmt clippy && \
    mkdir ~/.zfunc && \
    rustup completions zsh cargo > ~/.zfunc/_cargo

RUN \
    cargo install cargo-binstall && \
    cargo binstall -y cargo-watch cargo-feature
