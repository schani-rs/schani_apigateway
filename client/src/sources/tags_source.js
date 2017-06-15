export function newTag({ label }) {
  return {
    label: label || 'missing description',
  };
}

export function tag({ id, label }) {
  return {
    id: id || 0,
    label: label || 'missing description',
  };
}

export function getAll() {
  return [
    {
      id: 1,
      label: 'test1',
    },
    {
      id: 2,
      label: 'test2',
    },
    {
      id: 3,
      label: 'test3',
    },
    {
      id: 4,
      label: 'test4',
    },
  ];
}
