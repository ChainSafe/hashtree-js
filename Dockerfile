ARG NODE_VERSION

FROM matthewkeil/hashtree-debian-aarch64-env:${NODE_VERSION}

COPY . /usr/src/hashtree-js

WORKDIR /usr/src/hashtree-js/hashtree/src

RUN make

WORKDIR /usr/src/hashtree-js

RUN yarn config set supportedArchitectures.cpu "arm64" &&\
    yarn config set supportedArchitectures.libc "glibc" &&\
    yarn install

RUN yarn build --target aarch64-unknown-linux-gnu

RUN yarn test