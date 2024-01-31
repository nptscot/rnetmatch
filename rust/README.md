## Approximate Network Matching

The algorithm works like this: 

Let `A` and `B` be two vectors of `LineSring`, `Vec<LineString>`. 
Let `i` refer to the index position of a `LineString` in `A` and let `j` refer to the index position of `B`.
For each `LineString` in `A` or `B`, let the index of the component line be `k` where `Aik` is a `Line`.


- Initialize an empty R* Tree $Tree_A$
- Initialize an empty R* Tree $Tree_B$
- Initialize an empty `BTreeMap<usize, Vec<(usize, f64)>>`
- define a distance threshold `DT`
- define an angle threshold `AT`

```
for i in A:
  for k in i:
    calculate the slope of Aik
    insert Aik with a tuple of (i, slope_Aik) into Tree_A

for j in B:
  for k in j:
    calculate the slope of Bjk
    expand the AABB of Bjk in the x and y direction by DT
    insert Bjk with a tuple of (j, slope_Bjk)
```    
    

- Locate intersection candidates between A and B
- for each candidate pair, extract the slopes of Aik and Bjk
- calculate the angle of each slope using `atan(slope)`
- if the absolute difference between the angles of the slope is less than or equal to the slope, continue
- calculate the overlap in domain and range
- if there is overlap in domain and range, continue
- let `d` be the distance between lines `Aik` and `Bjk`
- if the distance between `Aik` and `Bjk` is less than `DT`, continue
- if the angle of `Ai` is less than or equal to 45
  - calculate the overlap in the x dimension between `Aik` and `Bjk` 
  - if there is overlap in the x-dimension
    - solve for y in the line defined by `Aik` based on `xmin` and `xmax`
    - calculate the length of the line segment defined by `(xmin, y1)` and `(xmax, y2)`
    - insert `i` into the BTreeMap if it does not exist
      - append (`j`, `d`) to the value vector if `j` does not exist 
      - if `j` is in the value vector, add `d` to the f64 value
- else if the angle of `Ai` is greater than 45 degrees
  - calculate the overlap in the y dimension between `Aik` and `Bjk`
  - if there is overlap in the y-dimension`
    - solve for x in the line defined by `Aik` based on `ymin` and `ymax`
    - calculate the length of the line segment defined by `(x1, ymin)` and `(x2, ymax)`
    - insert `i` into the BTreeMap if it does not exist
      - append (`j`, `d`) to the value vector if `j` does not exist 
      - if `j` is in the value vector, add `d` to the f64 value






