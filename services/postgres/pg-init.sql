CREATE USER docker;

CREATE DATABASE randomCompanyName;
GRANT ALL PRIVILEGES ON DATABASE randomCompanyName TO docker;
USE randomCompanyName;

CREATE TABLE `products` (
  `id` INT NOT NULL,
  `Name` VARCHAR(255) NOT NULL,
  `Price` INT NOT NULL,
  `Description` VARCHAR(255) NOT NULL,
  PRIMARY KEY (`id`)
);
INSERT INTO products (id, Name, Price, Description) VALUES (1, "Produto 1", 100, "Descrição do produto 1");
INSERT INTO products (id, Name, Price, Description) VALUES (2, "Produto 2", 200, "Descrição do produto 2");
