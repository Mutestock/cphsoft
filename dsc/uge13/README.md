<h4>1. Bayes</h4>
<ul>
    <li>A.
        <br>
        <img src="resources/screenshots/task_a_function.png"</img>
        <img src="resources/screenshots/task_a_result.png"</img>
    </li>
   <li>B.
        <br>
        <img src="resources/screenshots/task_b_function.png"</img>
        <img src="resources/screenshots/task_b_result.png"</img>
   </li>
   <li>C.
        <br>
        <img src="resources/screenshots/task_c_function.png"</img>
        <img src="resources/screenshots/task_c_result.png"</img>
   </li>
</ul>

<h4>2. </h4>
<p>
Make a KNN classifier on the IRIS dataset using Python. Make sure to split the dataset into training and
testing sets. 
</p>
<br>
<img src="resources/screenshots/knn_function.png">
<img src="resources/screenshots/knn_result_non_edgy.png">

<h4>3.</h4>
<p>Download KNIME. Make a KNN clasifier on the IRIS dataset</p>
<img src="resources/images/knime_knn.png">
<h4>4.</h4
<p>Find a dataset that interests you on the UCI Machine Learning Repository. Make a KNN classifier on the
dataset, either in Python or in KNIME. Visualize the classified data.</p>
<img src="resources/images/letters_confusion_matrix.png">
<img src="resources/images/letters_knn_accuracy.png">
<h4> 5 </h4>
<p>Implement a Gaussian Naïve Bayes classifier on the same data set (in either Python or KNIME).</p>
<img src="resources/images/letters_bayes.png">
<img src="resources/images/letters_naive_bayes_accuracy.png">
<img src="resources/images/letters_naive_bayes_confusion_matrix.png">
<h4> 6 </h4>
<p>Decide which of the two classifiers you would use on new data. What is behind your decision?</p>
<p>This explains it better than I can.: </p>
<p>https://www.datasciencecentral.com/profiles/blogs/comparing-classifiers-decision-trees-knn-naive-bayes</p>
<p></p>
<p>My subpar interpretation:</p>
<p></p>
<p>Consider bayes when:</p>
<p>Using big data when speed is a factor</p>
<p>High accuracy on big data is needed.</p>
<p>Less issues with dimensions and large feature sets.</p>
<p></p>
<p>Consider K-nn when:</p>
<p>Values which equals 0 breaks Bayes. They don't break k-nn</p>
<p>If the decision boundary isn't liniear, elliptical or parabolic, don't choose </p>bayes.
<p>Bayes requires underlying probability distributions. Knn does not.</p>
<p>Knn doesn't require training</p>