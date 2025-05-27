# frozen_string_literal: true

require_relative "markdown_it_ruby/version"
require_relative "markdown_it_ruby/markdown_it_ruby"

module MarkdownIt
  class Error < StandardError; end

  def self.convert(input, **options)
    options.transform_keys!(&:to_s)
    options.transform_values!(&:to_s)

    __convert(input, options)
  end
end
