FROM node:19-alpine
WORKDIR /app

RUN npm install -g pnpm
COPY build .npmrc package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile --prod

CMD ["node", "index.js"]
