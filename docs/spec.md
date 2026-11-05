What makes a program valid? 
What is an error? 
Is `1 +` valid? Is `-3` valid at Level 1? 
| Spec lists 5 valid and 5 invalid inputs with the reason for each invalid one

Valid inputs:
1. `1 + 2` - Valid because it is a complete arithmetic expression with two operands and an operator.
2. `3 * 4` - Valid because it is a complete arithmetic expression with two operands and an operator.
3. `(12 + 3) * (4 - 1)` - Valid because it is a complete arithmetic expression with parentheses, two operands, and operators.
4. `10 / 2` - Valid because it is a complete arithmetic expression with two operands and an operator.
5. `7 - 3` - Valid because it is a complete arithmetic expression with two operands and an operator.

Invalid inputs:
1. `1 +` - Invalid because it is an incomplete expression; it lacks a second operand after the operator.
2. `* 3` - Invalid because it starts with an operator without a preceding operand.
3. `4 / 0` - Invalid because division by zero is not allowed in arithmetic expressions.
4. `5 + 2 *` - Invalid because it is an incomplete expression; it   lacks a second operand after the last operator.
5. `-` - Invalid because it is not a complete expression; it lacks an operand
6. -3 - Invalid because it is not a complete expression; it lacks an operator and a second operand.