# Implementation Plan - Restore Functionalities & Metrics

The goal is to restore "lost" functionalities from the initial version while maintaining the new premium dark theme and tablet-optimized layout. Based on the backend capabilities and standard fitness app features, we will add missing metrics and ensure all controls are fully functional.

## User Review Required

> [!IMPORTANT]
> I have identified that **Speed**, **Distance**, and **Calories** are likely the missing features.
> - **Speed**: Available from the backend (`/status`).
> - **Distance**: Will be calculated on the frontend based on speed and time.
> - **Calories**: Will be calculated on the frontend based on power and time.
>
> **Question**: Are there any other specific features you are missing? (e.g., specific program types, user profiles, settings?)

## Proposed Changes

### 1. UI Layout Updates (`index.html`)
- **Expand Sidebar Metrics**:
    - Current: Cadence (RPM), Heure.
    - **Add**:
        - **Vitesse** (km/h) - From backend.
        - **Distance** (km) - Calculated.
        - **Calories** (kcal) - Calculated.
- **Layout Adjustment**:
    - Change `metrics-row` in the sidebar to accommodate 4-6 items, or add a second row.
    - Alternatively, move "Heure" to the Top Bar (replacing Date or combined) to free up space for workout metrics.

### 2. Frontend Logic Updates (`index.html` <script>)
- **State Management**:
    - Add variables for `totalDistance` and `totalCalories`.
    - Update `pollStatus` to read `speed`.
- **Calculations**:
    - **Distance**: Accumulate `speed * (time_delta)` or use average speed.
    - **Calories**: Accumulate based on `power` (Watts) * time. Formula: `Kcal = (Watts * Time_hours * 3.6) / 4.184` (approximate physics conversion) or a standard metabolic equivalent.
- **Persistence**:
    - Reset metrics when Timer/Program resets.

### 3. Verification
- Verify Speed updates from backend.
- Verify Distance/Calories increment correctly when pedaling (or mocking).
- Ensure layout remains touch-friendly on 11-inch tablet.

## Task List
- [x] **Update Sidebar Layout**: Modify HTML to include cards for Speed, Distance, and Calories. <!-- id: 0 -->
- [x] **Implement Logic**: Update JavaScript to fetch Speed and calculate Distance/Calories. <!-- id: 1 -->
- [x] **Refine Visuals**: Ensure new cards match the existing "glassmorphism" and dark theme style. <!-- id: 2 -->
