<h4> A. Bayes</h4>
<ul>
    <li>1.
        <br>
        <img src="resources/screenshots/task_a_function.png"</img>
        <img src="resources/screenshots/task_a_result.png"</img>
    </li>
   <li>2.
        <br>
        <img src="resources/screenshots/task_b_function.png"</img>
        <img src="resources/screenshots/task_b_result.png"</img>
   </li>
   <li>3.
        <br>
        <img src="resources/screenshots/task_c_function.png"</img>
        <img src="resources/screenshots/task_c_result.png"</img>
   </li>
</ul>




B. knn.py
C. ./resources/knime_knn.png
D. 
E.
F. This explains it better than I can: 
https://www.datasciencecentral.com/profiles/blogs/comparing-classifiers-decision-trees-knn-naive-bayes

Where Bayes Excels
1. Naive Bayes is a linear classifier while K-NN is not; It tends to be faster when applied to big data.   In comparison, k-nn is usually slower for large amounts of data, because of the calculations required for each new step in the process. If speed is important, choose Naive Bayes over K-NN.

2. In general, Naive Bayes is highly accurate when applied to big data. Don't discount K-NN when it comes to accuracy though; as the value of k in K-NN increases, the error rate decreases until it reaches that of the ideal Bayes (for k→∞).  

3. Naive Bayes offers you two hyperparameters to tune for smoothing: alpha and beta. A hyperparameter is a prior parameter that are tuned  on the training set to optimize it. In comparison, K-NN only has one option for tuning: the “k”, or number of neighbors. 

4. This method is not affected by the curse of dimensionality and large feature sets, while K-NN has problems with both.

5. For tasks like robotics and computer vision, Bayes outperforms decision trees.

Where K-nn Excels
1. If having conditional independence will highly negative affect classification, you'll want to choose K-NN over Naive Bayes. Naive Bayes can suffer from the zero probability problem; when a particular attribute's conditional probability equals zero, Naive Bayes will completely fail to produce a valid prediction. This could be fixed using a Laplacian estimator, but K-NN could end up being the easier choice.

2. Naive Bayes will only work if the decision boundary is linear, elliptic, or parabolic. Otherwise, choose K-NN.

3. Naive Bayes requires that you known the underlying probability distributions for categories. The algorithm compares all other classifiers against this ideal. Therefore, unless you know the probabilities and pdfs, use of the ideal Bayes is unrealistic. In comparison, K-NN doesn't require that you know anything about the underlying probability distributions.

4. K-NN doesn’t require any training—you just load the dataset and off it runs. On the other hand, Naive Bayes does require training.

5. K-NN (and Naive Bayes) outperform decision trees when it comes to rare occurrences. For example, if you're classifying types of cancer in the general population, many cancers are quite rare. A decision tree will almost certainty prune those important classes out of your model. If you have any rare occurrences, avoid using decision trees. 
