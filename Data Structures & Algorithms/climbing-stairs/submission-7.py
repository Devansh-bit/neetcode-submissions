class Solution:
    def climbStairs(self, n: int) -> int:
        def multiply(A, B):
            return [
                [
                    A[0][0] * B[0][0] + A[0][1] * B[1][0],
                    A[0][0] * B[0][1] + A[0][1] * B[1][1]
                ],
                [
                    A[1][0] * B[0][0] + A[1][1] * B[1][0],
                    A[1][0] * B[0][1] + A[1][1] * B[1][1]
                ]
            ]

        def matrix_power(M, p):
            # Identity matrix
            result = [[1, 0], [0, 1]]
            base = M

            while p > 0:
                if p & 1:
                    result = multiply(result, base)
                base = multiply(base, base)
                p >>= 1

            return result

        # M^n has F(n) at position [0][0]
        T = [[1, 1], [1, 0]]
        res = matrix_power(T, n)
        return res[0][0]