# frozen_string_literal: true

RSpec.describe MarkdownIt do
  it "has a version number" do
    expect(MarkdownIt::VERSION).to match(/\A\d+\.\d+\.\d+\z/)
  end

  describe 'MarkdownIt.convert' do
    subject { described_class.convert(input, **options) }

    let(:input) { "## title 1\nsome random markdown" }
    let(:options) {
      {
        internal_domain_name: 'https://example.com',
        heading_level_offset: 2,
        table_class_name:     'table-class',
      }
    }

    it 'returns a string' do
      expect(subject).to be_a(String)
    end

    it 'converts markdown to html' do
      expect(subject).to match(%r{\A<h4>title 1</h4>\n<p>some random markdown</p>\n\z})
    end

    context 'with fixtures' do
      let(:input) { File.read('spec/fixtures/dummy_input.md') }
      let(:expected_output) { File.read('spec/fixtures/dummy_output.html') }

      it 'converts markdown content to html' do
        expect(subject).to match(expected_output)
      end
    end
  end
end
