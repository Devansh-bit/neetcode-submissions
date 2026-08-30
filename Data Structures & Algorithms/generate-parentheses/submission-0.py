class Solution:
    def generateParenthesis(self, n: int) -> list[str]:
        result = []

        def explore(current_chars: list[str], open_count: int, close_count: int):
            if len(current_chars) == 2 * n:
                result.append("".join(current_chars))
                return
            
            if open_count < n:
                current_chars.append("(")
                explore(current_chars, open_count + 1, close_count)
                current_chars.pop()  # Backtrack
            
            if close_count < open_count:
                current_chars.append(")")
                explore(current_chars, open_count, close_count + 1)
                current_chars.pop()  # Backtrack

        explore([], 0, 0)
        return result