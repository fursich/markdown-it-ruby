# frozen_string_literal: true

RSpec.describe MarkdownIt do
  it "has a version number" do
    expect(MarkdownIt::VERSION).to match(/\A\d+\.\d+\.\d+\z/)
  end

  describe 'MarkdownIt.convert' do
    let(:input) { "# title 1\nsome random markdown" }

    context 'with options' do
      subject { described_class.convert(input, **options) }

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
        expect(subject).to match(%r{\A<h3>title 1</h3>\n<p>some random markdown</p>\n\z})
      end

      context 'with fixtures' do
        let(:input) { File.read('spec/fixtures/dummy_input.md') }
        let(:expected_output) { File.read('spec/fixtures/dummy_output_with_options.html') }

        it 'converts markdown content to html' do
          expect(subject).to eq(expected_output)
        end
      end
    end

    context 'without options' do
      subject { described_class.convert(input) }

      it 'returns a string' do
        expect(subject).to be_a(String)
      end

      it 'converts markdown to html' do
        expect(subject).to match(%r{\A<h1>title 1</h1>\n<p>some random markdown</p>\n\z})
      end

      context 'with fixtures' do
        let(:input) { File.read('spec/fixtures/dummy_input.md') }
        let(:expected_output) { File.read('spec/fixtures/dummy_output_default.html') }

        it 'converts markdown content to html' do
          expect(subject).to eq(expected_output)
        end
      end
    end
  end
end
