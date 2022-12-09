package main

type Queue struct {
	Items [3]int
	Min   int
	Max   int
}

func (q *Queue) Enqueue(i int) {
	if i > q.Max {
		q.Items[0], q.Items[1], q.Items[2] = q.Items[1], q.Items[2], i
		q.Max = q.Items[2]
	} else if i > q.Items[1] {
		q.Items[0], q.Items[1] = q.Items[1], i
	} else {
		q.Items[0] = i
	}
	q.Min = q.Items[0]
}

func (q *Queue) Sum() int {
	return q.Items[0] + q.Items[1] + q.Items[2]
}
