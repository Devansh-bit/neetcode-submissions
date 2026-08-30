class Solution:
    def generateParenthesis(self, n: int) -> list[str]:
        result = []

        def explore(current_string: List[str], open_count, close_count):
            if len(current_string) == 2*n:
                result.append("".join(current_string))
                return
            
            if open_count < n:
                current_string.append("(")
                explore(current_string, open_count+1, close_count)
                current_string.pop()

            if close_count < open_count:
                current_string.append(")")
                explore(current_string, open_count, close_count+1)
                current_string.pop()

        explore([], 0, 0)
        return result