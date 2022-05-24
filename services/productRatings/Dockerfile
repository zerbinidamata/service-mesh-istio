FROM ruby:3.0.0-alpine

ENV APP_HOME=/app
WORKDIR $APP_HOME

ARG PORT=3000
ENV PORT=${PORT}

COPY * $APP_HOME

CMD ["ruby", "ratings.rb"]
