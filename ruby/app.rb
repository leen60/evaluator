# httparty.rb
require 'httparty'
require 'colorize'

response = HTTParty.get('https://wix.com/_serverless/hiring-task-spreadsheet-evaluator/sheets')

class TokenKind
    IDENT = 'Identifier'.freeze
    STRING = 'String'.freeze
    INT = 'Int'.freeze 
    DOUBLE = 'Double'.freeze 
    ASSIGN = 'Assign'.freeze 
    BOOL = 'Bool'.freeze 
    PARENO = 'Op_Paren_o'.freeze 
    ARRO = 'Arr_o'.freeze 
    PARENC = 'Op_Paren_c'.freeze 
    ARRC = 'Arr_c'.freeze 
    COLON = 'Colon'.freeze
    BRACEO = 'OP_Brace_o'.freeze 
    BRACEC = 'OP_Brace_c'.freeze 
    SMEMICOL = 'Semicolon'.freeze 
    COMMA = 'Comma'.freeze 
    PUNC = 'Punc'.freeze 
    SUM = 'Operation_Sum'.freeze 
    MUL = 'Operation_Multiply'.freeze 
    DIV = 'Operation_Divide'.freeze
    AND = 'Operation_And'.freeze 
    OR = 'Operation_Or'.freeze 
    EQ = 'Operation_Eq'.freeze 
    NOT = 'Operation_Not'.freeze 
    CONCAT = 'Operation_Concat'.freeze 
    GT = 'Operation_Gt'.freeze 
    IF = 'Operation_If'.freeze
    ID = 'Id'.freeze 
    DATA = 'Data'.freeze 
    SHEETS = 'Sheets'.freeze 
    NOTATION = 'Notation'.freeze
    EOF ='Eof'.freeze
end

TokenKind. freeze

class Lexer

  KEYWORD = ['IF', 'AND', 'OR', 'SUM', 'DIVIDE', 'MULTIPLY', 'CONCAT', 'NOT', 'EQ', 'GT'].freeze
  BOOL = ['true', 'false'].freeze

  attr_reader :body, :counter, :offset, :id, :tokens, :buffer 

  def initialize(body)
	@body = body
	@counter = 0
     @offset = 0
     @id = 0
     @tokens = Array.new
     @buffer = Array.new
  end

   def new_token(kind)
	tokens.push(Token.new(kind, @buffer.reduce(:+), @id))
	reset
  end

  def new_int_token
  	tokens.push(Token.new(TokenKind::INT, @buffer.reduce(:+), @id))
	reset	
  end

  def new_float_token
  	tokens.push(Token.new(TokenKind::DOUBLE, @buffer.reduce(:+), @id))
	reset	
  end

  def new_string_token
  	tokens.push(Token.new(TokenKind::STRING, @buffer.reduce(:+), @id))
	reset
  end

  def new_ident_token
  	tokens.push(Token.new(TokenKind::IDENT, @buffer.reduce(:+), @id))
	reset	
  end

  def new_bool_token
  	tokens.push(Token.new(TokenKind::BOOL, @buffer.reduce(:+), @id))
	reset	
  end

  def is_digit?(s)
	code = s.ord
	48 <= code && code <= 57
  end

  def is_aphabetic?(s)
	code = s.ord
	65 <= code && code <= 90 || 97 <= code && code <= 122
  end

  def char_at
	self.body[self.offset]
  end

  def reset
	@buffer = []
  end

  def peek
  	next_index = @offset + 1
  	if next_index < @body.length
  		return @body[next_index]
  	end
  end

  def check_keywords
  	if KEYWORD.include?(@buffer.reduce(:+))
  		case @buffer.reduce(:+)
  		when 'IF'
  			tokens.push(Token.new(TokenKind::IF, buffer.reduce(:+), id))
  			reset
  			return
  		when 'SUM'
  			tokens.push(Token.new(TokenKind::SUM, buffer.reduce(:+), id))
  			reset
  			return
   		when 'DIVIDE'
  			tokens.push(Token.new(TokenKind::DIV, buffer.reduce(:+), id))
  			reset
  			return
   		when 'OR'
  			tokens.push(Token.new(TokenKind::OR, buffer.reduce(:+), id))
  			reset
  			return
   		when 'NOT'
  			tokens.push(Token.new(TokenKind::NOT, buffer.reduce(:+), id))
  			reset
  			return
   		when 'AND'
  			tokens.push(Token.new(TokenKind::AND, buffer.reduce(:+), id))
  			reset
  			return
   		when 'EQ'
  			tokens.push(Token.new(TokenKind::EQ, buffer.reduce(:+), id))
  			reset
  			return
    		when 'MULTIPLY'
  			tokens.push(Token.new(TokenKind::MUL, buffer.reduce(:+), id))
  			reset
  			return
    		when 'CONCAT'
  			tokens.push(Token.new(TokenKind::CONCAT, buffer.reduce(:+), id))
  			reset
  			return
    		when 'GT'
  			tokens.push(Token.new(TokenKind::GT, buffer.reduce(:+), id))
  			reset
  			return
  		end
  	end

  	if BOOL.include?(@buffer.reduce(:+))
  		tokens.push(Token.new(TokenKind::BOOL, buffer.reduce(:+), id))
  		reset
  		return
  	end

  	if @buffer.reduce(:+).match(/\A[A-Z]+[0-9]+\z/)
  		tokens.push(Token.new(TokenKind::NOTATION, buffer.reduce(:+), id))
  		reset
  		return
  	end
  end

  def check_insides
  	char_vec = @buffer.reduce(:+)
  	operation = Lexer.new(char_vec)
  	tok = operation.lex
  	i = 0
	while i < tok.length
		tokens.push(tok[i])
		i+=1
	end 
  end

  def lex_operation
  	@id+=1
  	@offset+=1
  	while @offset != @body.length
  		curr_char = char_at
  		if curr_char == '"'
  			check_insides
  			reset
  			break
  		elsif curr_char == '\\'
  			next_char = peek
  			if next_char == '"'
  				@buffer.push(curr_char)
  				@offset+=1
  				@buffer.push(next_char)
  				@offset+=1
  			else
  				@buffer.push(curr_char)
  				@offset+=1
  				break
  			end
  		elsif curr_char == '\n'
  			puts "Invalid char"
  			break
  		else
  			@buffer.push(curr_char)
  			@offset+=1
  		end	
  	end
  end

  def lex_string
  	@offset+=1
  	while offset != @body.length
  		curr_char = char_at
  		if curr_char == '"'
  			new_string_token
  			break
  		elsif curr_char == '\n'
  			puts "Invalid string #{curr_char}"
  			break
  		elsif curr_char == '\\'
  			@buffer.push('\"')
  			@offset+=1
  			new_string_token
  			break
  		else
  			@buffer.push(curr_char)
  			@offset+=1
  		end
  	end
  end

  def lex_int
  	while self.offset != self.body.length
  		curr_char = char_at
  		if is_digit?(curr_char)
  			self.buffer.push(curr_char)
  			@offset+=1
  		elsif is_aphabetic?(curr_char)
  			puts "Invalid int #{curr_char}, possition #{self.offset}"
  			break
  		elsif curr_char == '.'
  			self.buffer.push(curr_char)
  			@offset+=1
  			lex_float
  			break
  		else
  			new_int_token
  			@offset-=1
  			break
  		end
  	end 
  end

  def lex_float
  	while @offset != @body.length
  		curr_char = char_at
  		if is_digit?(curr_char)
  			@buffer.push(curr_char)
  			@offset+=1
  		elsif is_aphabetic?(curr_char)
  			puts "Invalid float #{curr_char}"
  		else 
  			new_float_token
  			@offset-=1
  			break
  		end
  	end
  end

  def lex_identifier
  	while self.offset != self.body.length
  		curr_char = char_at
  		if is_aphabetic?(curr_char) || is_digit?(curr_char) || curr_char == ' '
  			@buffer.push(curr_char)
  			@offset+=1
  		else
  			check_keywords
  			@offset-=1
  			break
  		end
  	end
  	check_keywords unless @buffer.empty?
  end

  def lex
  	while self.offset < self.body.length
  		c = char_at
		case c
		when "{"
		  new_token(TokenKind::BRACEO)
		  @id+=1
		when '}'
			new_token(TokenKind::BRACEC)
			@id+=1
		when '('
			  new_token(TokenKind::PARENO)
			  @id+=1
		when ')'
			new_token(TokenKind::PARENC)
			@id+=1
		when "="
			new_token(TokenKind::ASSIGN)
			@id+=1
		when '['
			new_token(TokenKind::ARRO)
			@id+=1
		when ']'
			new_token(TokenKind::ARRC)
			@id+=1
		when ':'
			new_token(TokenKind::PUNC)
			@id+=1
		when '"'
		  	if peek == '='
		  		lex_operation
		  	else 
		  		lex_string
		  	end
		when ','
			new_token(TokenKind::COMMA)
			@id+=1
		when "."
			new_token(TokenKind::COLON)
			@id+=1
		when ';' 
			new_token(TokenKind::SMEMICOL)
			@id+=1
		when '\\'
			next_char = peek
			if next_char == '"'
				@buffer.push('\"')
				@offset+=1
				lex_string
				@id+=1
			else
			 	print "#{c} is going to standard error!\n"
			end
		else
			if is_digit?(c)
				lex_int
			elsif is_aphabetic?(c)
				lex_identifier
			else
			   print "#{c} is going to standard error!\n" 
			end
		end
		@offset += 1
	end
	tokens
  end
end


class Token
	attr_reader :kind, :literal, :id

	def initialize(kind, literal, id)
		@kind = kind
		@literal = literal
		@id = id
	end

end

class Node
	attr_reader :type, :size, :parent, :stack_slot, :curr_stack_slot
end

class Spreadsheet < Node
	attr_reader :results

	def initialize(results)
		@results = results
	end
end

class AllSheets < Node
	attr_reader :results

	def initialize(results)
		@results = results
	end
end

class Data < Node
	attr_reader :results

	def initialize(results)
		@results = results
	end
end

class Sheet < Node
	attr_reader :id, :data

	def initialize(id, data)
		@id = id
		@data = data
	end
end

class DataCells < Node
	attr_reader :data

	def initialize(data)
		@data = data
	end
end

class Expression < Node
end

class ComplexExpressionNode < Expression
	attr_reader :literal, :slot, :arguments

	def initialize(literal, slot, arguments)
		@literal = literal
		@slot = slot
		@arguments = arguments
	end
end

class BoolNode < Expression
	attr_reader :literal, :slot

	def initialize(literal, slot)
		@literal = literal
		@slot = slot
	end
end

class DoubleNode < Expression
	attr_reader :literal, :slot

	def initialize(literal, slot)
		@literal = literal
		@slot = slot
	end
end

class StringNode < Expression
	attr_reader :literal, :slot

	def initialize(literal, slot)
		@literal = literal
		@slot = slot
	end
end

class IntNode < Expression
	attr_reader :literal, :slot

	def initialize(literal, slot)
		@literal = literal
		@slot = slot
	end
end

class NotationNode < Expression
	attr_reader :literal, :slot

	def initialize(literal, slot)
		@literal = literal
		@slot = slot
	end
end

class Parser
	attr_reader :tokens, :offset, :curr_token, :slot, :chars, :tiny_chars, :stack_slot, :flag
	
	def initialize(tokens)
		@tokens = tokens
		@offset = 0
		@curr_token = nil
		@slot = 0
		@chars = []
		@tiny_chars =[]
		@stack_slot = 1
		@flag = false
	end

	def get_current_token
		@curr_token = @tokens[@offset]
		@curr_token.kind
	end	

	def expect(kind)
		if get_current_token == kind
			@offset+=1
			return @curr_token
		else
			puts "Expected #{kind}, got #{@curr_token.kind}".red
		end
	end

	def parse_complex_expression(kind)
		@slot+=1
		terminal = @slot
		@flag = true
		node_operation_value = expect(kind)
		arguments = []
		expect(TokenKind::PARENO)
		if get_current_token != TokenKind::PARENC
			arguments.push(parse_operation)
			while get_current_token != TokenKind::PARENC
				expect(TokenKind::COMMA)
				arguments.push(parse_operation)
			end
		end
		@slot = terminal
		@flag = false
		expect(TokenKind::PARENC)
		return ComplexExpressionNode.new(node_operation_value, @slot, arguments)
	end

	def parse_operation
		case get_current_token
		when TokenKind::SUM
			return parse_complex_expression(TokenKind::SUM)
		when TokenKind::DIV
			return parse_complex_expression(TokenKind::DIV)
		when TokenKind::MUL
			return parse_complex_expression(TokenKind::MUL)
		when TokenKind::CONCAT
			return parse_complex_expression(TokenKind::CONCAT)
		when TokenKind::NOT
			return parse_complex_expression(TokenKind::NOT)
		when TokenKind::EQ
			return parse_complex_expression(TokenKind::EQ)
		when TokenKind::GT
			return parse_complex_expression(TokenKind::GT)
		when TokenKind::OR
			return parse_complex_expression(TokenKind::OR)
		when TokenKind::IF
			return parse_complex_expression(TokenKind::IF)
		when TokenKind::AND
			return parse_complex_expression(TokenKind::AND)
		when TokenKind::BOOL
			@slot+=1
			value = expect(TokenKind::BOOL)
			return BoolNode.new(value, @slot)
		when TokenKind::STRING
			@slot+=1
			value = expect(TokenKind::STRING)
			return StringNode.new(value, @slot)
		when TokenKind::NOTATION
			@slot+=1
			value = expect(TokenKind::NOTATION)
			return NotationNode.new(value, @slot)
		when TokenKind::INT
			@slot+=1
			value = expect(TokenKind::INT)
			return IntNode.new(value, @slot)
		when TokenKind::DOUBLE
			@slot+=1
			value = expect(TokenKind::DOUBLE)
			return DoubleNode.new(value, @slot)
		else 
			puts "bad node".red
		end

	end

	def parse_expr_sequence
		expect(TokenKind::ARRO)
		arguments = []

		if get_current_token == TokenKind::ASSIGN
			expect(TokenKind::ASSIGN)
			arguments.push(parse_operation)
		elsif get_current_token != TokenKind::ARRC
			arguments.push(parse_operation)
		end

		if get_current_token != TokenKind::ARRC
			while get_current_token != TokenKind::ARRC
				expect(TokenKind::COMMA)
				if get_current_token == TokenKind::ASSIGN
					expect(TokenKind::ASSIGN)
					arguments.push(parse_operation)
				else 
					arguments.push(parse_operation)
				end
			end
		end
		expect(TokenKind::ARRC)
		return DataCells.new(arguments)
	end

	def parse_sheet_data
		expect(TokenKind::ARRO)
		arguments = []

		if get_current_token == TokenKind::ARRO
        		arguments.push(parse_expr_sequence)
        	end

        # if self.current() != TokenKind::Arr_c { 
        #     while self.current() != TokenKind::Arr_c {
        #         self.expect(TokenKind::Comma);
        #         if self.current() == TokenKind::Arr_o {
        #             // self.expect(TokenKind::Arr_o);
        #             self.stack_slot+=1;
        #             args.push(self.parse_expr_sequence().unwrap());     
        #         }
        #     }
        # }
        	if get_current_token != TokenKind::ARRC
        		while get_current_token != TokenKind::ARRC
        			expect(TokenKind::COMMA)
        			if get_current_token == TokenKind::ARRO
        				arguments.push(parse_expr_sequence)
        			end
        		end
        	end
        	expect(TokenKind::ARRC)
        	return Data.new(arguments)
	end

	def parse_sheet
		arguments = []
		expect(TokenKind::BRACEO)
		expect(TokenKind::STRING)
		expect(TokenKind::PUNC)
		sheet_id = expect(TokenKind::STRING)
		expect(TokenKind::COMMA)
		expect(TokenKind::STRING)
		expect(TokenKind::PUNC)
		arguments.push(parse_sheet_data)
		expect(TokenKind::BRACEC)
		return Sheet.new(sheet_id, arguments)
	end

	def parse_all_sheets
		arguments =[]

        	expect(TokenKind::ARRO)
        	if get_current_token != TokenKind::ARRC
        		arguments.push(parse_sheet)
        		while get_current_token != TokenKind::ARRC
        			expect(TokenKind::COMMA)
        			if get_current_token == TokenKind::BRACEO
        				arguments.push(parse_sheet)
        			end
        		end
        	end

        	expect(TokenKind::ARRC)
        	return AllSheets.new(arguments)
	end

	def parse_all
		expect(TokenKind::BRACEO)
		expect(TokenKind::STRING)
		expect(TokenKind::PUNC)
		url = expect(TokenKind::STRING)
		expect(TokenKind::COMMA)
		sheets =expect(TokenKind::STRING)
		expect(TokenKind::PUNC)
		arguments = []
		while get_current_token != TokenKind::EOF
			arguments.push(parse_all_sheets)
            	expect(TokenKind::BRACEC)
		end
		return Spreadsheet.new(arguments)
	end
end

lexer = Lexer.new(response.body)
tok = lexer.lex
toky = []
toky.push(Token.new(TokenKind::BRACEO, "", 0))
toky.push(Token.new(TokenKind::STRING, "", 0))
toky.push(Token.new(TokenKind::PUNC, "", 0))
toky.push(Token.new(TokenKind::STRING, "", 0))
toky.push(Token.new(TokenKind::COMMA, "", 0))
toky.push(Token.new(TokenKind::STRING, "", 0))
toky.push(Token.new(TokenKind::PUNC, "", 0))

toky.push(Token.new(TokenKind::ARRO, "", 0))
toky.push(Token.new(TokenKind::BRACEO, "", 0))
toky.push(Token.new(TokenKind::STRING, "", 0))
toky.push(Token.new(TokenKind::PUNC, "", 0))
toky.push(Token.new(TokenKind::STRING, "", 0))
toky.push(Token.new(TokenKind::COMMA, "", 0))
toky.push(Token.new(TokenKind::STRING, "", 0))
toky.push(Token.new(TokenKind::PUNC, "", 0))
toky.push(Token.new(TokenKind::ARRO, "", 0))
toky.push(Token.new(TokenKind::ARRO, "", 0))
toky.push(Token.new(TokenKind::BOOL, "true", 0))
toky.push(Token.new(TokenKind::COMMA, "", 0))
toky.push(Token.new(TokenKind::ASSIGN, "", 0))
toky.push(Token.new(TokenKind::SUM, "SUM", 0))
toky.push(Token.new(TokenKind::PARENO, "", 0))
toky.push(Token.new(TokenKind::NOTATION, "A5", 0))
toky.push(Token.new(TokenKind::PARENC, "", 0))
toky.push(Token.new(TokenKind::ARRC, "", 0))
toky.push(Token.new(TokenKind::ARRC, "", 0))
toky.push(Token.new(TokenKind::BRACEC, "", 0))
toky.push(Token.new(TokenKind::ARRC, "", 0))
toky.push(Token.new(TokenKind::BRACEC, "", 0))
toky.push(Token.new(TokenKind::EOF, "", 0))


tok.push(Token.new(TokenKind::EOF, "", 0))
parser = Parser.new(tok)
tree = parser.parse_all
puts tree.results.inspect.red