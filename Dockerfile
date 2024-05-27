ARG NODE_VERSION

FROM matthewkeil/hashtree-debian-aarch64-env:${NODE_VERSION}

ARG WORKSPACE_DIR

COPY . /usr/src/hashtree-js

# COPY ${WORKSPACE_DIR}/.cargo-cache/git/db /usr/local/cargo/git/db 
# COPY ${WORKSPACE_DIR}/.cargo/registry/cache /usr/local/cargo/cache 
# COPY ${WORKSPACE_DIR}/.cargo/registry/index /usr/local/cargo/registry/index 

WORKDIR /usr/src/hashtree-js/hashtree/src

RUN make

WORKDIR /usr/src/hashtree-js

RUN yarn config set supportedArchitectures.cpu "arm64" &&\
    yarn config set supportedArchitectures.libc "glibc" &&\
    yarn install

RUN yarn build --target aarch64-unknown-linux-gnu

RUN yarn test