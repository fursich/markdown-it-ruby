# frozen_string_literal: true

require_relative "lib/markdown_it_ruby/version"

Gem::Specification.new do |spec|
  spec.name = "markdown_it_ruby"
  spec.version = MarkdownIt::VERSION
  spec.authors = ["Koji Onishi"]
  spec.email = ["fursich0@gmail.com"]
  spec.summary = "ruby integration of markdown-it-rust (with custom plugins)"
  spec.description = ""
  spec.homepage = "https://github.com/fursich/markdown_it_ruby"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.1.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/fursich/markdown_it_ruby"
  # spec.metadata["changelog_uri"] = "TODO: Put your gem's CHANGELOG.md URL here."

  # Specify which files 0hould be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) { |ls|
    ls.readlines("\x0", chomp: true).reject { |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .github appveyor Gemfile])
    }
  }
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/markdown_it_ruby/Cargo.toml"]

  # Uncomment to register a new dependency of your gem
  spec.add_dependency "rb_sys", "~> 0.9.108"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
  spec.metadata['rubygems_mfa_required'] = 'true'
end
