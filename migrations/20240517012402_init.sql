-- Add migration script here
CREATE OR REPLACE FUNCTION update_modified_column() 
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW; 
END;
$$ language 'plpgsql';

CREATE TABLE customers (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    contact_number VARCHAR(15) NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL
);
CREATE TRIGGER update_customer_modtime BEFORE UPDATE ON customers FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE orders (
    id SERIAL PRIMARY KEY NOT NULL,
    customer_id INT NOT NULL,
    order_status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id)
);
CREATE TRIGGER update_order_modtime BEFORE UPDATE ON orders FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE items (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NULL,
    quantity_available INT NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL
);
CREATE TRIGGER update_item_modtime BEFORE UPDATE ON items FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE order_items (
    order_id INT NOT NULL,
    item_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    PRIMARY KEY (order_id, item_id),
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (item_id) REFERENCES items(id)
);
CREATE TRIGGER update_order_item_modtime BEFORE UPDATE ON order_items FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE vendors (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    contact_number VARCHAR(15) NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL
);
CREATE TRIGGER update_vendor_modtime BEFORE UPDATE ON vendors FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE vehicles (
    id SERIAL PRIMARY KEY NOT NULL,
    vendor_id INT NOT NULL,
    type VARCHAR(50) NOT NULL,
    capacity DECIMAL(10, 2) NOT NULL,
    availability_status BOOLEAN NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    FOREIGN KEY (vendor_id) REFERENCES vendors(id)
);
CREATE TRIGGER update_vehicle_modtime BEFORE UPDATE ON vehicles FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE routes (
    id SERIAL PRIMARY KEY NOT NULL,
    main_route_id INT NULL,
    origin VARCHAR(100) NOT NULL,
    destination VARCHAR(100) NOT NULL,
    distance DECIMAL(10, 2) NOT NULL,
    estimated_travel_time TIME NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL
);
CREATE TRIGGER update_route_modtime BEFORE UPDATE ON routes FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE vehicle_routes (
    vehicle_id INT NOT NULL,
    route_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    PRIMARY KEY (vehicle_id, route_id),
    FOREIGN KEY (vehicle_id) REFERENCES vehicles(id),
    FOREIGN KEY (route_id) REFERENCES routes(id)
);
CREATE TRIGGER update_vehicle_route_modtime BEFORE UPDATE ON vehicle_routes FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(100) NOT NULL,
    contact_number VARCHAR(15) NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL
);
CREATE TRIGGER update_user_modtime BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE TABLE order_users (
    order_id INT NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    PRIMARY KEY (order_id, user_id),
    FOREIGN KEY (order_id) REFERENCES orders(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE TRIGGER update_order_user_modtime BEFORE UPDATE ON order_users FOR EACH ROW EXECUTE PROCEDURE update_modified_column();