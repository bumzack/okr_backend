FROM node:21

ADD . /build
WORKDIR /build
RUN npm install

RUN ["npm", "run", "build"]

EXPOSE 12120
