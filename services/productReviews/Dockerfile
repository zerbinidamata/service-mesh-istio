FROM ruby:latest-slim

ENV APP_HOME=/app
ARG PORT=3000
ENV PORT=${PORT}

COPY * $APP_HOME

WORKDIR $APP_HOME

CMD ["ruby", "reviews.rb"]
