# Diesel
Usage: diesel.exe [OPTIONS] <COMMAND>

Commands:
  migration     A group of commands for generating, running, and reverting migrations.
  setup         Creates the migrations directory, creates the database specified in your DATABASE_URL, and runs existing migrations.
  database      A group of commands for setting up and resetting your database.
  completions   Generate shell completion scripts for the diesel command.
  print-schema  Print table definitions for database schema.
  help          Print this message or the help of the given subcommand(s)

Options:
      --database-url <DATABASE_URL>
          Specifies the database URL to connect to. Falls back to the DATABASE_URL environment variable if unspecified.

      --config-file <CONFIG_FILE>
          The location of the configuration file to use. Falls back to the `DIESEL_CONFIG_FILE` environment variable if unspecified. Defaults to `diesel.toml` in your project root. See diesel.rs/guides/configuring-diesel-cli for documentation on this file.

      --locked-schema
          When `print_schema.file` is specified in your config file, this flag will cause Diesel CLI to error if any command would result in changes to that file. It is recommended that you use this flag when running migrations in CI or production.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

You can also run `diesel SUBCOMMAND -h` to get more information about that subcommand.

# Diesel migration
A group of commands for generating, running, and reverting migrations.

Usage: diesel.exe migration [OPTIONS] <COMMAND>

Commands:
  run       Runs all pending migrations.
  revert    Reverts the specified migrations.
  redo      Reverts and re-runs the latest migration. Useful for testing that a migration can in fact be reverted.
  list      Lists all available migrations, marking those that have been applied.
  pending   Returns true if there are any pending migrations.
  generate  Generate a new migration with the given name, and the current timestamp as the version.
  help      Print this message or the help of the given subcommand(s)

Options:
      --database-url <DATABASE_URL>
          Specifies the database URL to connect to. Falls back to the DATABASE_URL environment variable if unspecified.
      --migration-dir <MIGRATION_DIRECTORY>
          The location of your migration directory. By default this will look for a directory called `migrations` in the current directory and its parents.
      --config-file <CONFIG_FILE>
          The location of the configuration file to use. Falls back to the `DIESEL_CONFIG_FILE` environment variable if unspecified. Defaults to `diesel.toml` in your project root. See diesel.rs/guides/configuring-diesel-cli for documentation on this file.
      --locked-schema
          Require that the schema file is up to date.
  -h, --help
          Print help (see more with '--help')

## Diesel migration revert
Reverts the specified migrations.

Usage: diesel.exe migration revert [OPTIONS]

Options:
  -a, --all
          Reverts previously run migration files.
      --database-url <DATABASE_URL>
          Specifies the database URL to connect to. Falls back to the DATABASE_URL environment variable if unspecified.
      --migration-dir <MIGRATION_DIRECTORY>
          The location of your migration directory. By default this will look for a directory called `migrations` in the current directory and its parents.
      --config-file <CONFIG_FILE>
          The location of the configuration file to use. Falls back to the `DIESEL_CONFIG_FILE` environment variable if unspecified. Defaults to `diesel.toml` in your project root. See diesel.rs/guides/configuring-diesel-cli for documentation on this file.
  -n, --number <REVERT_NUMBER>
          Reverts the last `n` migration files. [default: 1]
      --locked-schema
          Require that the schema file is up to date.
  -h, --help
          Print help (see more with '--help')
