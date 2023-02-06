FROM node:19-alpine AS build
WORKDIR /app
COPY . .
RUN yarn
RUN yarn build

FROM node:19-alpine AS release
WORKDIR /app
COPY --from=build /app/build .
COPY --from=build /app/package.json .
RUN yarn --prod

CMD ["node", "index.js"]
