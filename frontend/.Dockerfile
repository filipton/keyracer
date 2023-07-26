FROM node:19-alpine
WORKDIR /app

RUN npm install -g pnpm
COPY build .npmrc package.json ./
RUN pnpm install --frozen-lockfile --prod

CMD ["node", "index.js"]
