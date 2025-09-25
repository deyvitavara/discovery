-- Crear DB y tablas mínimas para la demo
IF DB_ID('SalesDb') IS NULL
BEGIN
  CREATE DATABASE SalesDb;
END
GO

USE SalesDb;
GO

IF OBJECT_ID('dbo.Customers','U') IS NULL
BEGIN
    CREATE TABLE dbo.Customers(
        Id INT IDENTITY(1,1) PRIMARY KEY,
        Name NVARCHAR(100) NOT NULL,
        Email NVARCHAR(100) NOT NULL
    );
END
GO

IF OBJECT_ID('dbo.Products','U') IS NULL
BEGIN
    CREATE TABLE dbo.Products(
        Id INT IDENTITY(1,1) PRIMARY KEY,
        Name NVARCHAR(100) NOT NULL,
        Price DECIMAL(18,2) NOT NULL DEFAULT(0)
    );
END
GO

IF OBJECT_ID('dbo.Orders','U') IS NULL
BEGIN
    CREATE TABLE dbo.Orders(
        Id INT IDENTITY(1,1) PRIMARY KEY,
        CustomerId INT NOT NULL FOREIGN KEY REFERENCES dbo.Customers(Id),
        OrderDate DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME(),
        Total DECIMAL(18,2) NOT NULL DEFAULT(0)
    );
END
GO
