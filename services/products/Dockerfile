FROM node:latest-slim

ENV APP_HOME /app
ARG PORT=8080
ENV PORT=${PORT}

COPY * $APP_HOME

WORKDIR $APP_HOME

RUN npm install

EXPOSE $PORT

CMD ["node", "products.js"]