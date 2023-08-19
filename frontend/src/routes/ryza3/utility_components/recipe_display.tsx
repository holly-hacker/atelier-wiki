import types from "@/data/types/ryza3";
import * as d3 from "d3";
import { useEffect, useRef, useState } from "react";

// the radius of a ring, where the distance between 2 rings is 2 units
const ringRadius = 0.6;

// the margin around the edge of the canvas, in units
const data_margin = ringRadius * 1.5;

// the size of the arrows between rings, in units
const arrow_size = 0.125;

// the angle of the arrow fins, in degrees
const arrow_angle = 60;

// whether to use a pseudo-hexagonal grid
const use_hex_grid = true;

export default function RecipeDisplay({ recipe }: { recipe: types.Recipe }) {
  const svgRef = useRef(null);
  const [k, setK] = useState(1);
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);

  const field_index = 0; // TODO: only showing the first field for now
  const field = recipe.fields[field_index];

  const width = 640;
  const height = 480;

  const [scale_x, scale_y, scale_rel] = create_scales(field, width, height);

  const elementColors = ["red", "blue", "yellow", "green"];

  useEffect(() => {
    const zoom = d3.zoom().on("zoom", (event) => {
      const { x, y, k } = event.transform;
      setK(k);
      setX(x);
      setY(y);
    });
    // @ts-ignore: probably incorrect typedefs
    d3.select(svgRef.current).call(zoom);
  }, []);

  return (
    <svg
      ref={svgRef}
      width={width}
      height={height}
      style={{ border: "1px solid black" }}
    >
      <rect width="100%" height="100%" fill="#f8f8f8"></rect>
      <g
        fill="transparent"
        stroke="currentColor"
        strokeWidth="1.5"
        transform={`translate(${x} ${y}) scale(${k})`}
      >
        {/* Draw lines for dependencies */}
        <RingConnectionLines
          field={field}
          scale_x={scale_x}
          scale_y={scale_y}
          scale_rel={scale_rel}
        />

        {/* Draw rings */}
        {field.map((ring, i) => (
          <circle
            key={i}
            cx={scale_x(ring.x)}
            cy={scale_y(ring.y)}
            r={scale_rel(ringRadius)}
            color={elementColors[ring.element]}
            fill={ring.required ? "#fff" : "#ccc"}
            strokeWidth={scale_rel(ring.required ? 0.06 : 0.03)}
          />
        ))}
      </g>
    </svg>
  );
}

function create_scales(
  field: types.Ring[],
  width: number,
  height: number,
): [
  d3.ScaleLinear<number, number>,
  d3.ScaleLinear<number, number>,
  d3.ScaleLinear<number, number>,
] {
  const aspect_ratio = width / height;

  const data_x = field.map((f) => f.x);
  const data_y = field.map((f) => f.y);
  const [min_x, max_x] = [
    Math.min(...data_x) - data_margin,
    Math.max(...data_x) + data_margin,
  ];
  const [min_y, max_y] = [
    Math.min(...data_y) - data_margin,
    Math.max(...data_y) + data_margin,
  ];

  // while the rings are placed on a 2D grid, they are used as if they were on a hex grid
  // to make this come through in our scale, we multiply the aspect ratio of the data to by 1.67
  const data_width = max_x - min_x;
  const data_height = max_y - min_y;
  const data_aspect_ratio =
    (data_width / data_height) * (use_hex_grid ? 5 / 3 : 1);

  const adjust_x =
    Math.max(1, aspect_ratio / data_aspect_ratio) * data_width - data_width;
  const adjust_y =
    Math.max(1, data_aspect_ratio / aspect_ratio) * data_height - data_height;

  // NOTE: Y scale is inverted because 0,0 is top-left in-game
  const scale_x = d3.scaleLinear(
    [min_x - adjust_x / 2, max_x + adjust_x / 2],
    [0, width],
  );
  const scale_y = d3.scaleLinear(
    [max_y + adjust_y / 2, min_y - adjust_y / 2],
    [height, 0],
  );

  // x and y should have equal scales (when accounted for aspect ratio skew of a hex grid), so use
  // one of the scales to create a "relative" scale that can be used for sizing rather than
  // positioning.
  // this assertion is mostly a sanity check and to make it clearer what is happening.
  const step_x = (scale_x(1) - scale_x(0)) / (use_hex_grid ? 5 / 3 : 1);
  const step_y = scale_y(1) - scale_y(0);
  console.assert(
    Math.abs(step_x - step_y) < 0.0001,
    "x and y scales should be equal (%o vs %o)",
    step_x,
    step_y,
  );

  const scale_relative = d3.scaleLinear([0, 1], [0, step_y]);

  return [scale_x, scale_y, scale_relative];
}

function RingConnectionLines({
  field,
  scale_x,
  scale_y,
  scale_rel,
}: {
  field: types.Ring[];
  scale_x: d3.ScaleLinear<number, number>;
  scale_y: d3.ScaleLinear<number, number>;
  scale_rel: d3.ScaleLinear<number, number>;
}) {
  const path_segment = field
    .map((ring) => {
      if (!ring.predecessor) {
        return "";
      }

      const line_xy = d3.line(
        (d) => scale_x(d[0]),
        (d) => scale_y(d[1]),
      );

      const offsets = [
        [0, -2],
        [+1, -1],
        [+1, +1],
        [0, +2],
        [-1, +1],
        [-1, -1],
      ];

      const offset = offsets[ring.predecessor.direction];

      const source: [number, number] = [ring.x, ring.y];
      const target: [number, number] = [ring.x + offset[0], ring.y + offset[1]];

      let line_drawing = line_xy([source, target]);

      // also draw an arrow pointing to the target
      if (ring.required) {
        // don't draw arrows for core loops, they don't really depend on eachother
      } else {
        // get the coordinates of the midpoint of the line
        const mid_point: [number, number] = [
          scale_x(ring.x + offset[0] / 2),
          scale_y(ring.y + offset[1] / 2),
        ];

        const base_angle =
          (use_hex_grid
            ? ring.predecessor.direction * 60
            : [0, 45, 135, 180, 225, 315][ring.predecessor.direction]) - 90;
        const arrow_drawing = get_arrow_drawing(
          mid_point,
          base_angle,
          scale_rel,
        );

        line_drawing += " " + arrow_drawing;
      }

      return line_drawing;
    })
    .join(" ");

  return <path d={path_segment}></path>;
}

function get_arrow_drawing(
  pos: [number, number],
  angle: number,
  scale: d3.ScaleLinear<number, number>,
) {
  const to_rad = (deg: number) => (deg / 180) * Math.PI;

  const arrow: [number, number][] = [
    [
      pos[0] + scale(Math.cos(to_rad(angle + arrow_angle))) * arrow_size,
      pos[1] + scale(Math.sin(to_rad(angle + arrow_angle))) * arrow_size,
    ],
    [
      pos[0] - scale(Math.cos(to_rad(angle))) * arrow_size,
      pos[1] - scale(Math.sin(to_rad(angle))) * arrow_size,
    ],
    [
      pos[0] + scale(Math.cos(to_rad(angle - arrow_angle))) * arrow_size,
      pos[1] + scale(Math.sin(to_rad(angle - arrow_angle))) * arrow_size,
    ],
  ];

  return d3.line()(arrow);
}
