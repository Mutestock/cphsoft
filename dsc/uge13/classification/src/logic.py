

def bayes_hockey():
    # A. 60% of the kids play football, and 36% of the kids play ice hockey. 40% of the kids who play football
    # also play ice hockey. What percent of those that play ice hockey also play football?

    # 3 Known values

    # P = probability of
    # F = football
    # I = ice_hockey

    P_F = 0.6
    P_I = 0.36
    P_FI = 0.4

    # Bayes: P(A|B) = (P(B|A) * P(A)) / P(B)
    # Translated: P(I|F) = (P(F|I)*P(F))/P(I)
    # Or: P(ice/foot) = (P(foot/ice)*P(foot))/P(ice)

    P_IF = (P_FI * P_F) / P_I
    print(
        f"\nProbability of ice hockey players also playing football: {round(P_IF*100, 2)}%")


def bayes_music_dance():

    # B. 40% of the kids like music, and 24% of the kids like to dance. Given that 30% of those that like
    # music also likes to dance, what percent of those that like to dance also likes music?

    # 3 Known values

    # P = probability of
    # M = music
    # D = dance

    P_M = 0.4
    P_D = 0.24
    P_MD = 0.3

    # Bayes: P(A|B) = P(B|A) = (P(B|A) * P(A)) / P(B)
    # Translated: P(D|M) = (P(M|D)*P(M))/P(D)
    # Or: P(dance/music) = (P(music/dance)*P(music))/P(dance)

    P_DM = (P_MD * P_M) / P_D
    print(f"\nProbability of dancers also liking music = {round(P_DM*100, 2)}%")


def bayes_factory():
    """
    C. In a factory, machine X produces 60% of the daily output and machine Y produces 40% of the daily
    output.
    2% of machine X’s output is defective, and 1.5% of machine Y’s output is defective.
    One day, an item is inspected at random, and found to be defective. What is the probability that it
    was produced by machine X?
    """
    # 4 known values
    # Probability of = P
    # machine_x = A
    # machine_y = B
    # defect = D

    P_A = 0.6
    P_B = 0.4
    P_DA = 0.02
    P_DB = 0.015

    # Bayes: P(A|B) = P(B|A) = (P(B|A) * P(A)) / P(B)
    # Expanded from example from disease ~>
    # P(A|D) = (P(D|A)*P(A)) / ((P(D|A) * P(A)) * (P(D|B) * P(B)))

    P_AD = (P_DA * P_A) / ((P_DA * P_A) + (P_DB * P_B))
    print(
        f"\nProbability that the bulb was created at machineX = {round(P_AD*100, 2)}%")


if __name__ == "__main__":
    bayes_hockey()
    bayes_music_dance()
    bayes_factory()
