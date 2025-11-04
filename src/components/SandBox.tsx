import { extend, useTick } from "@pixi/react";
import type { Ball, Trail, Universe } from "physics-engine";
import { Container, Graphics } from "pixi.js";
import { useCallback, useRef, useState } from "react";
import { useSimulation } from "../contexts/SimulationContext";
extend({
  Container,
  Graphics,
});

interface SandBoxProps {
  universe: Universe;
}

export default function SandBox({ universe }: SandBoxProps) {
  const {
    selectedballIndex,
    setSelectedballIndex,
    setIsPropertyEditorOpen,
    render,
    setRender,
  } = useSimulation();

  const [hoveredballIndex, setHoveredballIndex] = useState<number | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [rotationIndicator, setRotationIndicator] = useState(0);
  const pixiContainerRef = useRef<any>(null);

  const handleballDragStart = (index: number, event: any) => {
    event.stopPropagation();
    setSelectedballIndex(index);
    setIsDragging(true);
    // Pause simulation while dragging
    universe.set_is_paused(true);
  };

  const handleballDrag = (index: number, event: any) => {
    if (!isDragging) return;

    const balls = universe.get_balls();

    const localPos = pixiContainerRef.current.toLocal(event.global);

    // Get the position relative to the previous ball or origin
    const prevX = index === 0 ? 0 : balls[index - 1].pos.x;
    const prevY = index === 0 ? 0 : balls[index - 1].pos.y;

    // Calculate new angle from mouse position
    const dx = localPos.x - prevX;
    const dy = localPos.y - prevY;
    const newTheta = Math.atan2(dx, dy);

    // Update the ball's angle
    universe.update_ball_theta(index, newTheta);
    setRender((prev) => prev + 1);
  };

  const handleballDragEnd = () => {
    setIsDragging(false);
    // Resume simulation
    if (!universe.get_is_paused()) {
      universe.set_is_paused(false);
    }
  };

  const drawCallback = useCallback(
    (graphics: any) => {
      graphics.clear();

      const balls: Ball[] = universe.get_balls();
      const trails: Trail[][] = universe.get_trails();
      const isPaused = universe.get_is_paused();

      // Draw angle arcs, velocity and acceleration vectors when paused
      if (isPaused) {
        for (let i = 0; i < balls.length; i++) {
          const prevX = i === 0 ? 0 : balls[i - 1].pos.x;
          const prevY = i === 0 ? 0 : balls[i - 1].pos.y;

          // Draw angle arc
          const arcRadius = 40;
          const theta = balls[i].theta;

          // console.log("Drawing angle arc for ball", i, "theta:", theta);

          // Draw arc from downward vertical (0) to theta
          // In the coordinate system, 0 degrees is straight down (Math.PI/2 in standard coords)
          const endAngle = Math.PI / 2; // Straight down (0 degrees in pendulum coords)
          const startAngle = Math.PI / 2 - theta; // Current angle
          graphics.moveTo(balls[i].pos.x, balls[i].pos.y);

          graphics.setStrokeStyle({
            width: 2,
            color: 0x4a5568,
            alpha: 0.8,
            cap: "round",
            join: "round",
          });
          // graphics.clear();
          graphics.arc(
            prevX,
            prevY,
            arcRadius,
            startAngle,
            endAngle,
            theta < 0 // counterclockwise if theta is negative
          );
          graphics.stroke();

          // Calculate velocity (v = ω * r for circular motion)
          const rodLength = balls[i].rod ? balls[i].rod.length : 100;
          const velocityMag = Math.abs(balls[i].omega * rodLength);
          const velocityAngle =
            theta + (balls[i].omega > 0 ? Math.PI / 2 : -Math.PI / 2);

          // Draw velocity vector (tangent to motion)
          if (velocityMag > 0.1) {
            const velScale = 1.0; // Increased scale for better visibility
            const velX = Math.sin(velocityAngle) * velocityMag * velScale;
            const velY = Math.cos(velocityAngle) * velocityMag * velScale;

            graphics.setStrokeStyle({
              width: 3,
              color: 0x4a5568,
              alpha: 1,
              cap: "round",
              join: "round",
            });
            graphics.moveTo(balls[i].pos.x, balls[i].pos.y);
            graphics.lineTo(balls[i].pos.x + velX, balls[i].pos.y + velY);
            graphics.stroke();

            // Draw arrow head for velocity
            const arrowSize = 15;
            const arrowAngle = Math.PI / 6;
            const velAngleDir = Math.atan2(velY, velX);

            // Arrow tip is at the end of the velocity vector
            const tipX = balls[i].pos.x + velX;
            const tipY = balls[i].pos.y + velY;

            // Move the arrow base back along the shaft
            const arrowOffset = -8; // Distance back from the tip
            const arrowTipX = tipX - Math.cos(velAngleDir) * arrowOffset;
            const arrowTipY = tipY - Math.sin(velAngleDir) * arrowOffset;

            graphics.poly([
              {
                x: arrowTipX,
                y: arrowTipY,
              },
              {
                x: arrowTipX - Math.cos(velAngleDir - arrowAngle) * arrowSize,
                y: arrowTipY - Math.sin(velAngleDir - arrowAngle) * arrowSize,
              },
              {
                x: arrowTipX - Math.cos(velAngleDir + arrowAngle) * arrowSize,
                y: arrowTipY - Math.sin(velAngleDir + arrowAngle) * arrowSize,
              },
            ]);
            graphics.fill({ color: 0x4a5568, alpha: 1 });
          }
        }
      }

      // Draw center ball
      graphics.circle(0, 0, 10);
      graphics.fill({ color: 0x0f0f0f });

      // Draw rods
      for (let i = 0; i < balls.length; i++) {
        if (balls[i].rod) {
          graphics.setStrokeStyle({
            width: 2,
            color: balls[i].rod!.color,
            alpha: 1,
          });
          graphics.moveTo(
            i == 0 ? 0 : balls[i - 1].pos.x,
            i == 0 ? 0 : balls[i - 1].pos.y
          );
          graphics.lineTo(balls[i].pos.x, balls[i].pos.y);
          graphics.stroke();
        }
      }

      // Draw trails
      for (let i = 0; i < trails.length; i++) {
        const trail = trails[i];
        for (let j = 0; j < trail.length; j++) {
          graphics.setStrokeStyle({
            width: 2,
            color: trail[j].color,
            alpha: (j + 1) / trail.length,
          });
          if (j == 0) {
            graphics.moveTo(trail[j].pos.x, trail[j].pos.y);
          } else {
            graphics.lineTo(trail[j].pos.x, trail[j].pos.y);
          }
          graphics.stroke();
        }
      }

      // Draw balls with hover highlight
      for (let i = 0; i < balls.length; i++) {
        // Draw hover highlight (only when paused)
        if (hoveredballIndex === i && isPaused) {
          graphics.circle(balls[i].pos.x, balls[i].pos.y, balls[i].radius + 8);
          graphics.fill({ color: 0xffaa00, alpha: 0.1 });
          graphics.stroke({ width: 3, color: 0xffaa00, alpha: 0.6 });
        }

        // Draw ball
        graphics.circle(balls[i].pos.x, balls[i].pos.y, balls[i].radius);
        graphics.fill({ color: balls[i].color, alpha: 1 });
      }
    },
    [render, hoveredballIndex, selectedballIndex, rotationIndicator]
  );

  useTick((delta) => {
    if (universe.get_is_paused()) return;
    setRender(render + 1);
    universe.time_step(delta.deltaTime);

    // Update rotation indicator for selected ball
    if (selectedballIndex !== null) {
      const balls = universe.get_balls();
      if (balls[selectedballIndex]) {
        setRotationIndicator(
          (prev) =>
            prev + balls[selectedballIndex].omega * delta.deltaTime * 0.1
        );
      }
    }
  });
  return (
    <>
      <pixiContainer
        x={0}
        y={0}
        ref={pixiContainerRef}
        interactive
        onPointerDown={(e: any) => {
          // Only allow interaction when paused
          if (!universe.get_is_paused()) return;

          const localPos = pixiContainerRef.current.toLocal(e.data.global);
          const balls = universe.get_balls();

          // Check if clicked on a ball first
          for (let i = 0; i < balls.length; i++) {
            const dx = localPos.x - balls[i].pos.x;
            const dy = localPos.y - balls[i].pos.y;
            const dist = Math.sqrt(dx * dx + dy * dy);

            if (dist < balls[i].radius + 10) {
              setSelectedballIndex(i);
              // Only open property editor when paused
              if (universe.get_is_paused()) {
                setIsPropertyEditorOpen(true);
              }
              handleballDragStart(i, e);
              return;
            }
          }

          // Check if clicked on a rod
          for (let i = 0; i < balls.length; i++) {
            const prevX = i === 0 ? 0 : balls[i - 1].pos.x;
            const prevY = i === 0 ? 0 : balls[i - 1].pos.y;
            const ballX = balls[i].pos.x;
            const ballY = balls[i].pos.y;

            // Calculate distance from point to line segment
            const A = localPos.x - prevX;
            const B = localPos.y - prevY;
            const C = ballX - prevX;
            const D = ballY - prevY;

            const dot = A * C + B * D;
            const lenSq = C * C + D * D;
            const param = lenSq !== 0 ? dot / lenSq : -1;

            let nearestX, nearestY;

            if (param < 0) {
              nearestX = prevX;
              nearestY = prevY;
            } else if (param > 1) {
              nearestX = ballX;
              nearestY = ballY;
            } else {
              nearestX = prevX + param * C;
              nearestY = prevY + param * D;
            }

            const distToRod = Math.sqrt(
              (localPos.x - nearestX) ** 2 + (localPos.y - nearestY) ** 2
            );

            if (distToRod < 10) {
              // Clicked on rod - select the ball it's attached to
              setSelectedballIndex(i);
              // Only open property editor when paused
              if (universe.get_is_paused()) {
                setIsPropertyEditorOpen(true);
              }
              return;
            }
          }

          // Clicked on empty space - deselect
          setSelectedballIndex(null);
        }}
        onGlobalPointerMove={(e: any) => {
          if (isDragging && selectedballIndex !== null) {
            handleballDrag(selectedballIndex, e);
          }

          // Only allow hover highlighting when paused
          if (!universe.get_is_paused()) {
            setHoveredballIndex(null);
            pixiContainerRef.current.cursor = "default";
            return;
          }

          if (!isDragging) {
            // Check if hovering over a ball and update hover state
            const balls = universe.get_balls();
            let hoveringOverball = false;
            let hoveredIndex: number | null = null;
            const localPos = pixiContainerRef.current.toLocal(e.data.global);

            for (let i = 0; i < balls.length; i++) {
              const dx = localPos.x - balls[i].pos.x;
              const dy = localPos.y - balls[i].pos.y;
              const dist = Math.sqrt(dx * dx + dy * dy);
              if (dist < balls[i].radius + 10) {
                hoveringOverball = true;
                hoveredIndex = i;
                break;
              }
            }

            setHoveredballIndex(hoveredIndex);
            pixiContainerRef.current.cursor = hoveringOverball
              ? "grab"
              : "default";
          }
        }}
        onPointerUp={handleballDragEnd}
        onPointerUpOutside={handleballDragEnd}
      >
        <pixiGraphics draw={drawCallback} />
      </pixiContainer>
    </>
  );
}
