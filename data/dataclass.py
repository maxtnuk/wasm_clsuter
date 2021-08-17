from sklearn.datasets import make_classification
from sklearn.datasets import make_blobs
from sklearn.datasets import make_gaussian_quantiles

import numpy as np


def save(name, arr):
    np.savetxt(f"./data/{name}.csv", arr,fmt='%.6f,%.6f,%.0f', header="X,Y,Class", comments="")


def format(X1, Y1):
    Y1 = np.reshape(Y1, (-1, 1))
    return np.hstack((X1, Y1))

X1, Y1 = make_classification(
    n_features=2, n_redundant=0,class_sep=2, n_informative=2, n_clusters_per_class=1, n_classes=3)


save('classification', format(X1, Y1))

X1, Y1 = make_blobs(n_features=2, centers=3,center_box=(0,0))


save('blob', format(X1, Y1))

X1, Y1 = make_gaussian_quantiles(n_features=2, n_classes=3)

save('gaussian', format(X1, Y1))
