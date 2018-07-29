require 'prawn'
Prawn::Document.generate('outline.pdf') do
  # First we create 10 pages and some default outline
  (1..10).each do |index|
    text "Page #{index}"
    start_new_page
  end

  outline.define do
    section('Section 1', destination: 1) do
      page title: 'Page 2', destination: 2
      page title: 'Page 3', destination: 3
    end
  end
end
