
entity.gen:
	sea-orm-cli generate entity \
	  -l \
	  --with-serde both \
	  --serde-skip-deserializing-primary-key \
	  --serde-skip-hidden-column \
	  -o entity/src

migrate.init:
	sea-orm-cli migrate init

migrate.gen:
	sea-orm-cli migrate generate user

migrate.up:
	sea-orm-cli migrate up

migrate.refresh:
	sea-orm-cli migrate refresh