FROM node:18-alpine

WORKDIR /app/client
COPY ./public ./public
COPY ./src ./src
COPY ./package.json .

RUN npm install

CMD ["npm", "run", "start"]