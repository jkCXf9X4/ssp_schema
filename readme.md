

git submodule add https://github.com/modelica/ssp-standard 3rdParty/SSP_Schema


# Generate rust headers


## https://github.com/ProgVal/rust-xml-schema

wget https://www.w3.org/2009/XMLSchema/XMLSchema.xsd -O xml-schema/XMLSchema.xsd
wget https://www.w3.org/2009/XMLSchema/derived.nxsd -O ./xml-schema/derived.nxsd
cargo run --package xml-schema --bin gen xml-schema/derived.nxsd xml-schema/XMLSchema.xsd > foo.rs
cp foo.rs xml-schema/src/parser.rs
cargo test

cargo run --package xml-schema --bin gen ssp/SystemStructureCommon.xsd ssp/SystemStructureDescription11.xsd > ssd.rs

någonting poppar ut iaf....

inte så snyggt men något



## XMLSchemer
does not work

cargo run -- -i ssp/schema/SystemStructureDescription11.xsd -o ssd.rs