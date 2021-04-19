
# === Examples. It's not for you, reviewer. Shoo ===

def bayes_disease_example():
    # https://www.analyzemath.com/probabilities/bayes-theorem.html
    pop_disease = 0.01
    pop_clean = 0.99
    test_accuracy = 0.95
    test_falsely_positive = 0.02

    # Question: If a person selected at random from the population has tested positive,
    # What is the probability of the person being infected?

    # D = Have the disease
    # FD = free from disease (ND?)
    # TP = test is positive

    # P(D|TP) = (P(TP|D)*P(D)) / ((P(TD|D) * P(D)) + (P(TP|ND) * P(ND)))

    bayes = (test_accuracy * pop_disease) / \
        ((test_accuracy * pop_disease)+test_falsely_positive*pop_clean)

    print(bayes)


def bayes_ball_example():
    # https://www.youtube.com/watch?v=k6Dw0on6NtM&ab_channel=Dr.TreforBazett
    bucket1 = 6/3
    bucket2 = 6/2

    # A: Select a blue ball
    # P = Probability
    # P(A|b1) = The probability of choosing a blue ball given that we are looking in bucket1
    # Bucket one has 6 balls total. 3 of which are blue.
    # P(a|b1) = 3/6 ~ 1/2

    probability_of_blue_ball_when_bucket_one = 1/2

    # Bucket two has 6 balls total. 2 of which are blue
    # P(a|b2) = 2/6 ~ 1/3

    probability_of_blue_ball_when_bucket_two = 1/3

    # It's equally likely that the buckets are being drawn from
    # P(b1) = P(b2) = 1/2

    probability_of_choosing_either_bucket = 1/2
    '''
    P(A) = The probability of choosing a blue ball.
    Disjoint union of sample space. b1 and b2 are separate. There are no overlaps. 
    P(a) = The probability of choosing a blue ball, given that that we're looking at bucket1 + the probability of choosing a blue blue ball given that we're looking at bucket2
    https://decodedscience.org/introducing-math-symbols-for-union-and-intersection/
    P(a) = P(A∩B1) + P(A∩B2)
    i.e. the probability of a disjoint union is the sum of those probabilities.
    P(a|b) = P(A∩B) / P(B)
    P(a)  = P(A∩B1) + P(A∩B2)
          = P(A|B1) * P(B) + P(A|B2) * P(B2)
          = 1/2 * 1/2 + 1/3 * 1/2
          = 1/4 + 1/6
          = 3/12 + 2/12
          = 5/12
          

    Bayes:
    P(B1/A) = (P(A|B1) * P(B1)) / P(A)
            = (1/2 * 1/2) / 5/12
            = 3/5
            
    I.e. when having drawn a ball, the chance of that ball coming from bucket1 is 3/5          
    '''


def bayes_gender_example():
    # https://www.youtube.com/watch?v=XQoLVl31ZfQ&ab_channel=Dr.TreforBazett
    # Simplest case
    probability_of_girl = 0.25
    probability_of_boy = 0.25

    # P(A|B) = P(A∩B) / P(B)
    # P(B|A) = P(B∩A) / P(A)
    # Bayes: P(A|B) = P(B|A) = (P(B|A) * P(A)) / P(B)

    # Question: Given that one of the kids will be a girl, what is the chance of getting two girls?
    # P(2 girls | At least 1 girl)
    #       = (P(At least 1 girl | given that you get two girls) * P(2 girls)) / P(1 Girl)
    # There's a 100 % chance that you get at least 1 girl
    # There's a 1/4 chance of getting two girls
    # There's a 3/4 chance of getting two girls, when we know that there are one girl
    #       = (1 * 1/4) / 3/4
    #       = 1/3
    #
    # Can also be "verified" like this:
    # [[GG], GB, BG], BB