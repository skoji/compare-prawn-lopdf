require 'prawn'

Prawn::Document.generate('hello2.pdf') do
  font("Courier") do
    font_size 48
    draw_text "Hello World!", :at => [100,600]
  end
end
